use std::error::Error as ErrTrait;

pub use eventstore::{Connection, Credentials};
pub use uuid::Uuid;

pub mod db;
pub mod context;
pub mod permissions;

pub use context::Context;

pub type Error = Box<dyn ErrTrait>;