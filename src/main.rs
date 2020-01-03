extern crate failure;
extern crate pretty_env_logger;
extern crate juniper;
extern crate warp;
extern crate juniper_warp;

use std::sync::Arc;
use juniper::RootNode;
use jwks_client::keyset::KeyStore;
use backend_rust::graphql::{context::Context, resolvers::{Query, Mutation}};
use serde::{Serialize};
use eventstore::{Connection, Credentials};
use std::net::SocketAddr;
use backend_rust::auth::JWTAuth;
use warp::{
    http::HeaderMap,
    http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_HEADERS},
    Filter
};

const JWKS_URL: &str = "JWKS_URL";
const JWT_ISSUER: &str = "JWT_ISSUER";
const ENDPOINT_URL: &str = "ENDPOINT_URL";
const GRAPHQL_PLAYGROUND: &str = "GRAPHQL_PLAYGROUND";

#[derive(Serialize)]
struct JsonError {
    error: String
}

struct Config {
    pub port: u16,
    pub database_url: SocketAddr,
    pub database_login: String,
    pub database_password: String,
    pub playground_enabled: bool,
    pub jwks_url: String,
    pub issuer: String,
    pub audience: String
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
        let playground_enabled = std::env::var(GRAPHQL_PLAYGROUND).unwrap_or("false".to_string()).parse::<bool>().unwrap();
        let jwks_url = std::env::var(JWKS_URL).expect("JWKS_URL must be set");
        let issuer = std::env::var(JWT_ISSUER).expect("JWT_ISSUER must be set");
        let audience = std::env::var(ENDPOINT_URL).expect("ENDPOINT_URL must be set");

        Config {
            port,
            database_url, database_login, database_password,
            playground_enabled,
            jwks_url,
            issuer, audience
        }
    }
}

type Schema = RootNode<'static, Query, Mutation>;

fn schema() -> Schema {
    Schema::new(Query, Mutation)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let config = Config::from_env();
    let addr: SocketAddr = ([127, 0, 0, 1], config.port).into();

    let log = warp::log("warp_server");

    log::info!("Listening on {}", addr.to_string());

    let db_conn = Arc::new(
        Connection::builder()
            .with_default_user(Credentials::new(config.database_login, config.database_password))
            .single_node_connection(config.database_url)
    );

    let key_store = KeyStore::new_from(config.jwks_url.as_str()).expect("Failed to create JWKS key store");
    let auth = Arc::new(JWTAuth::new(key_store, config.issuer, config.audience));

    let no_auth = {
        let db_conn = db_conn.clone();
        warp::any().map(move || -> Context{
            Context {
                db: db_conn.clone(),
                user: None
            }
        })
    };
    let context = {
        let auth = auth.clone();
        let db_conn = db_conn.clone();
        warp::header::<String>("Authorization")
            .map(move |jwt: String| -> Context {
                let auth = auth.clone();
                let user = auth.validate(jwt).unwrap();
                Context {
                    db: db_conn.clone(),
                    user: Some(user)
                }
            })
            .or(no_auth)
            .unify()
    };

    let graphql_filter = juniper_warp::make_graphql_filter_async(schema(), context.boxed());
    let headers = headers();

    warp::serve(
        warp::get2()
            .and(warp::path::end())
            .and(juniper_warp::playground_filter("/graphql"))
            .or(warp::path("graphql").and(graphql_filter).with(warp::reply::with::headers(headers)))
            .with(log)
    ).run(addr)
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    headers
}