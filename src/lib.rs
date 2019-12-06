// Used for testing

#[macro_use] extern crate failure;
#[macro_use] extern crate diesel;
extern crate juniper;

pub mod permissions;
pub mod context;
pub mod db;
pub mod schema;
pub mod resolvers;