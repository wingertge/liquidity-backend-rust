#[macro_use]
extern crate tracing;
#[macro_use]
extern crate liquidity_macros;
#[macro_use]
extern crate serde;

mod models;
pub mod repository;
pub mod resolvers;
pub mod schema;

pub use resolvers::ElectionResolvers;
