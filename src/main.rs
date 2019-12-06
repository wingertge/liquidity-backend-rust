#[macro_use] extern crate failure;
#[macro_use] extern crate diesel;
extern crate futures;
extern crate hyper;
extern crate juniper;
extern crate juniper_hyper;
extern crate pretty_env_logger;

use std::sync::Arc;
use juniper::RootNode;
use hyper::service::service_fn;
use hyper::{rt::{self, Future}, Method, Response, Body, StatusCode, Server, Request};
use futures::future;
use hyper::http::header::AUTHORIZATION;
use jwks_client::keyset::KeyStore;
use crate::resolvers::{Query, Mutation};
use futures::future::FutureResult;
use failure::Error;
use crate::JWTError::{InvalidJWTFormat, InvalidRequestFormat, InvalidSignature};
use db::DbPool;
use crate::db::create_db_pool;
use serde::{Serialize};
use serde_json::Value;

mod schema;
mod resolvers;
mod db;

const JWKS_URL: &str = "JWKS_URL";
const JWT_ISSUER: &str = "JWT_ISSUER";
const ENDPOINT_URL: &str = "ENDPOINT_URL";
const GRAPHQL_PLAYGROUND: &str = "GRAPHQL_PLAYGROUND";

pub struct Context {
    pub db: DbPool,
    pub user: Option<Box<User>>
}

pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}

#[derive(Debug, Fail, Serialize)]
#[allow(clippy::enum_variant_names)]
enum JWTError {
    #[fail(display = "JWT Token failed to validate")]
    InvalidSignature,
    #[fail(display = "Invalid request: {}", reason)]
    InvalidRequestFormat {reason: String},
    #[fail(display = "Invalid JWT format: {}", reason)]
    InvalidJWTFormat {reason: String}
}

#[derive(Serialize)]
struct JsonError {
    error: String
}

fn get_user(req: &Request<Body>, keys: Arc<KeyStore>) -> Result<Option<Box<User>>, Error> {
    let issuer = std::env::var(JWT_ISSUER).expect("JWT_ISSUER must be set");
    let audience = std::env::var(ENDPOINT_URL).expect("ENDPOINT_URL must be set");

    let jwt: Option<String> = match req.headers().get(AUTHORIZATION) {
        Some(header) => {
            let header_string = header.to_str().map_err(|_| InvalidRequestFormat {reason: "Authorization header isn't a string".to_string()})?;
            Some(header_string.replace("Bearer ", ""))
        },
        None => None
    };

    match jwt {
        Some(jwt) => {
            let decoded = keys.verify(jwt.as_str()).map_err(|_| InvalidSignature)?;
            let audiences = decoded.payload().get_array("aud");
            let audience_valid = match audiences {
                Some(audiences) => {
                    let audiences: Vec<String> = audiences.iter()
                        .map(|x| Ok(x.as_str().ok_or(InvalidJWTFormat { reason: "Audiences array contains non strings".to_string() })?.to_string()))
                        .collect::<Result<Vec<String>, Error>>()?;
                    audiences.contains(&audience)
                },
                None => decoded.payload().aud().ok_or(InvalidJWTFormat {reason: "Missing audience from JWT".to_string()})? == audience
            };
            let issuer_valid = decoded.payload().iss().ok_or(InvalidJWTFormat {reason: "Missing issuer from JWT".to_string()})? == issuer;

            if !issuer_valid || !audience_valid {
                Err(InvalidJWTFormat {reason: "Token wasn't issued for this service!".to_string()}.into())
            }
            else {
                let id = decoded.payload().sub()
                    .ok_or(InvalidJWTFormat {reason: "Missing subject from JWT".to_string()})?
                    .to_string();
                let empty = Vec::<Value>::new();
                let permissions = decoded.payload()
                    .get_array("permissions")
                    .unwrap_or_else(|| &empty)
                    .iter()
                    .map(|x| x.as_str().expect("Can't convert permission to string").to_string())
                    .collect();
                Ok(Some(Box::new(User {
                    id,
                    permissions
                })))
            }
        },
        _ => Ok(None)
    }
}

fn get_request_context(req: &Request<Body>, context: Arc<Context>, keys: Arc<KeyStore>) -> Result<Arc<Context>, Error> {
    let user = get_user(req, keys)?;
    Ok(Arc::new(Context { user, db: context.db.clone() }))
}

fn send_error(e: Error) -> Box<FutureResult<Response<Body>, hyper::Error>> {
    let error = JsonError {error: format!("{:?}", e)};
    let json = serde_json::to_string(&error).expect("Serializing this type should never fail");
    let mut response = Response::new(Body::from(json));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    Box::new(future::ok(response))
}

fn main() {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 4000).into();
    let db_pool = create_db_pool();
    let root_node = Arc::new(RootNode::new(Query, Mutation));
    let jwt_keys = Arc::new(KeyStore::new_from(std::env::var(JWKS_URL)
            .expect("JWKS_URL must be set").as_str()).unwrap());
    let ctx = Arc::new(Context {db: db_pool, user: None});
    let playground = std::env::var(GRAPHQL_PLAYGROUND).unwrap_or("false".to_string()).parse::<bool>().unwrap();
    println!("Playground {}.", if playground {"enabled"} else {"disabled"});

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = ctx.clone();
        let jwt_keys = jwt_keys.clone();
        service_fn(move |req: Request<Body>| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => {
                    if playground { Box::new(juniper_hyper::playground("/graphql")) }
                    else {
                        let mut response = Response::new(Body::empty());
                        *response.status_mut() = StatusCode::NOT_FOUND;
                        Box::new(future::ok(response))
                    }
                },
                (&Method::GET, "/graphql") => {
                    let ctx = get_request_context(&req, ctx.clone(), jwt_keys.clone());
                    match ctx {
                        Ok(ctx) => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                        Err(e) => {
                            println!("{:?}", e);
                            send_error(e)
                        }
                    }
                },
                (&Method::POST, "/graphql") => {
                    let ctx = get_request_context(&req, ctx.clone(), jwt_keys.clone());
                    match ctx {
                        Ok(ctx) => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                        Err(e) => send_error(e)
                    }
                },
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };

    let server = Server::bind(&addr).serve(new_service).map_err(|e| println!("Server error: {}", e));
    println!("Listening on http://{}", addr);
    rt::run(server)
}