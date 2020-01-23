#[macro_use] extern crate async_trait;
#[macro_use] extern crate tracing;

use std::error::Error as ErrTrait;

pub use eventstore::{Connection, Credentials};
pub use uuid::Uuid;

pub mod db;
pub mod context;
pub mod permissions;

pub use context::Context;

pub type Error = Box<dyn ErrTrait>;

pub trait Merge<T> {
    fn merge_with(self, new: T) -> Self;
}