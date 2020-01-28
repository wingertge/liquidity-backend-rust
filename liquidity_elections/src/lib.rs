#[macro_use] extern crate tracing;
#[macro_use] extern crate liquidity_macros;
#[macro_use] extern crate serde;

pub mod repository;
pub mod resolvers;
pub mod schema;
mod models;

pub use resolvers::ElectionResolvers;