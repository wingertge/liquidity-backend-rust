#[macro_use]
extern crate tracing;
#[macro_use]
extern crate juniper;

mod auth;
mod mutation;
mod query;

use crate::auth::JWTError;
use crate::{auth::JWTAuth, mutation::Mutation, query::Query};
use hyper::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
use juniper::RootNode;
use jwks_client::keyset::KeyStore;
use liquidity::{Connection, Credentials};
use liquidity_api::{APIContext, ElectionResolvers};
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};

const JWKS_URL: &str = "JWKS_URL";
const JWT_ISSUER: &str = "JWT_ISSUER";
const ENDPOINT_URL: &str = "ENDPOINT_URL";
const GRAPHQL_PLAYGROUND: &str = "GRAPHQL_PLAYGROUND";

struct Config {
    pub port: u16,
    pub database_url: SocketAddr,
    pub database_login: String,
    pub database_password: String,
    pub playground_enabled: bool,
    pub jwks_url: String,
    pub issuer: String,
    pub audience: String,
    pub cache_size: usize,
    pub cache_ttl: Duration
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .map(|x| x.parse::<u16>())
            .unwrap_or(Ok(4000))
            .expect("Invalid port set in environment");
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set")
            .parse()
            .expect("DATABASE_URL must be a valid socket address");
        let database_login = std::env::var("DATABASE_LOGIN").expect("DATABASE_LOGIN must be set");
        let database_password =
            std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let playground_enabled = std::env::var(GRAPHQL_PLAYGROUND)
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap();
        let jwks_url = std::env::var(JWKS_URL).expect("JWKS_URL must be set");
        let issuer = std::env::var(JWT_ISSUER).expect("JWT_ISSUER must be set");
        let audience = std::env::var(ENDPOINT_URL).expect("ENDPOINT_URL must be set");
        let cache_size = std::env::var("CACHE_MAX_SIZE")
            .unwrap_or_else(|_| "500".to_string())
            .parse()
            .expect("Invalid cache size");
        let cache_ttl = std::env::var("CACHE_TIME_TO_LIVE")
            .map(|x| parse_duration::parse(x.as_str()).expect("Invalid cache TTL"))
            .unwrap_or_else(|_| Duration::from_secs(600));

        Config {
            port,
            database_url,
            database_login,
            database_password,
            playground_enabled,
            jwks_url,
            issuer,
            audience,
            cache_size,
            cache_ttl
        }
    }
}

fn init_tracing() {
    use opentelemetry::{api::Provider, sdk};
    use tracing_opentelemetry::OpentelemetryLayer;
    use tracing_subscriber::{Layer, Registry};

    // Create tracer
    let tracer = sdk::Provider::default().get_tracer("liquidity_backend");

    // Create tracing layer
    let layer = OpentelemetryLayer::with_tracer(tracer);
    let subscriber = layer.with_subscriber(Registry::default());

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

type Schema = RootNode<'static, Query, Mutation>;

fn schema() -> Arc<Schema> {
    Arc::new(Schema::new(Query, Mutation))
}
fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    headers
}

fn render_auth_error(err: JWTError) -> Response<Body> {
    let message = format!("{}", err);
    let errors = vec![serde_json::json!({ "message": message })];
    let json = serde_json::json!({ "errors": errors }).to_string();
    let resp = Response::new(Body::from(json));
    resp
}

fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::NOT_FOUND;
    Ok(response)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    init_tracing();

    let config = Config::from_env();
    let addr: SocketAddr = ([127, 0, 0, 1], config.port).into();

    let db_conn = Arc::new(
        Connection::builder()
            .with_default_user(Credentials::new(
                config.database_login.clone(),
                config.database_password.clone()
            ))
            .with_connection_name("eventstore")
            .single_node_connection(config.database_url)
            .await
    );

    let key_store = KeyStore::new_from(config.jwks_url.as_str())
        .await
        .expect("Failed to create JWKS key store");
    let auth = Arc::new(JWTAuth::new(
        key_store,
        config.issuer.clone(),
        config.audience.clone()
    ));
    let elections = Arc::new(ElectionResolvers::new(config.cache_size, config.cache_ttl));
    let base_ctx = APIContext::new(db_conn, None, elections);
    let schema = schema();
    let playground_enabled = config.playground_enabled.clone();

    #[instrument(skip(req, schema, auth))]
    async fn handle_request(
        req: Request<Body>,
        schema: Arc<Schema>,
        ctx: APIContext,
        auth: Arc<JWTAuth>,
        playground_enabled: bool,
        _remote_addr: SocketAddr
    ) -> Result<Response<Body>, hyper::Error> {
        let res: Result<_, hyper::Error> = match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                if playground_enabled {
                    juniper_hyper::playground("/graphql").await
                } else {
                    not_found()
                }
            }
            (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                let user = req.headers().get(AUTHORIZATION).map(|value| {
                    let token = value.to_str().unwrap().to_string();
                    auth.validate(token)
                });
                let ctx = user
                    .map(|res| res.map(|user| ctx.clone_with_user(user)))
                    .unwrap_or_else(|| Ok(ctx.clone()));

                match ctx {
                    Ok(ctx) => {
                        let mut response =
                            juniper_hyper::graphql_async(schema, Arc::new(ctx), req).await?;
                        *response.headers_mut() = headers();
                        Ok(response)
                    }
                    Err(err) => {
                        error!("{}", err);
                        Ok(render_auth_error(err))
                    }
                }
            }
            (&Method::OPTIONS, "/graphql") => {
                let mut response = Response::new(Body::empty());
                *response.headers_mut() = headers();
                Ok(response)
            }
            _ => not_found()
        };
        res
    }

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let schema = schema.clone();
        let ctx = base_ctx.clone();
        let auth = auth.clone();
        let playground_enabled = playground_enabled.clone();
        let remote_addr = conn.remote_addr();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(
                    req,
                    schema.clone(),
                    ctx.clone(),
                    auth.clone(),
                    playground_enabled.clone(),
                    remote_addr.clone()
                )
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    info!("Listening on {}", addr.to_string());

    if let Err(e) = server.await {
        error!("server error: {}", e)
    }
}
