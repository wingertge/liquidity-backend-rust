#[macro_use] extern crate tracing;
extern crate juniper;
extern crate warp;
extern crate juniper_warp;
extern crate tracing_subscriber;

mod query;
mod mutation;
mod auth;

use std::{sync::Arc, net::SocketAddr};
use juniper::RootNode;
use jwks_client::keyset::KeyStore;
use crate::{auth::{JWTAuth, JWTError}, query::Query, mutation::Mutation};
use warp::{
    Filter,
    http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN},
    http::HeaderMap
};
use liquidity::{Connection, Credentials};
use liquidity_api::{APIContext, ElectionResolvers};
use std::time::Duration;

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
        let database_password = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let playground_enabled = std::env::var(GRAPHQL_PLAYGROUND).unwrap_or_else(|_| "false".to_string()).parse::<bool>().unwrap();
        let jwks_url = std::env::var(JWKS_URL).expect("JWKS_URL must be set");
        let issuer = std::env::var(JWT_ISSUER).expect("JWT_ISSUER must be set");
        let audience = std::env::var(ENDPOINT_URL).expect("ENDPOINT_URL must be set");
        let cache_size = std::env::var("CACHE_MAX_SIZE").unwrap_or_else(|_| "500".to_string()).parse().expect("Invalid cache size");
        let cache_ttl = std::env::var("CACHE_TIME_TO_LIVE")
            .map(|x| parse_duration::parse(x.as_str()).expect("Invalid cache TTL"))
            .unwrap_or_else(|_| Duration::from_secs(600));

        Config {
            port,
            database_url, database_login, database_password,
            playground_enabled,
            jwks_url,
            issuer, audience,
            cache_size, cache_ttl
        }
    }
}

fn init_tracing() {
    use opentelemetry::{api::Provider, sdk};
    use tracing_opentelemetry::OpentelemetryLayer;
    use tracing_subscriber::{Layer, Registry};

    // Create tracer
    let tracer = sdk::Provider::default()
        .get_tracer("liquidity_backend");

    // Create tracing layer
    let layer = OpentelemetryLayer::with_tracer(tracer);
    let subscriber = layer.with_subscriber(Registry::default());

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

type Schema = RootNode<'static, Query, Mutation>;

fn schema() -> Schema {
    Schema::new(Query, Mutation)
}
fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    headers
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    init_tracing();

    let config = Config::from_env();
    let addr: SocketAddr = ([127, 0, 0, 1], config.port).into();

    let log = warp::log("warp_server");

    info!("Listening on {}", addr.to_string());

    let db_conn = Arc::new(
        Connection::builder()
            .with_default_user(Credentials::new(config.database_login, config.database_password))
            .single_node_connection(config.database_url)
            .await
    );

    let key_store = KeyStore::new_from(config.jwks_url.as_str()).await.expect("Failed to create JWKS key store");
    let auth = Arc::new(JWTAuth::new(key_store, config.issuer, config.audience));
    let elections = Arc::new(ElectionResolvers::new(config.cache_size, config.cache_ttl));
    let base_ctx = APIContext::new(db_conn, None, elections);

    let no_auth = {
        let ctx = base_ctx.clone();
        warp::any().map(move || -> Result<APIContext, JWTError> {
            Ok(ctx.clone())
        })
    };
    let context = {
        warp::header::<String>("Authorization")
            .map(move |jwt: String| -> Result<APIContext, JWTError> {
                let auth = auth.clone();
                let user = auth.validate(jwt)?;

                Ok(base_ctx.clone_with_user(user))
            })
            .or(no_auth)
            .unify()
    };

    let options = warp::options().map(warp::reply).with(warp::reply::with::headers(headers()));
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), context.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path::end())
            .and(juniper_warp::playground_filter("/graphql"))
            .or(warp::path("graphql").and(graphql_filter).with(warp::reply::with::headers(headers())))
            .or(options)
            .with(log)
    ).run(addr)
}