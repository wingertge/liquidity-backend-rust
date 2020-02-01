#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate tracing;

use std::error::Error as ErrTrait;

pub use eventstore::{Connection, Credentials};
pub use uuid::Uuid;

pub mod context;
pub mod db;
pub mod permissions;

pub use context::Context;
use std::fmt::{Debug, Display};

pub type Error = Box<dyn ErrTrait>;

pub trait Merge<T> {
    fn merge_with(self, new: T) -> Self;
}

pub trait Loggable: Sized {
    fn log_value(self) -> Self;
}

impl<T: Debug, E: Display> Loggable for Result<T, E> {
    fn log_value(self) -> Self {
        match self {
            Ok(ref result) => debug!("{:?}", result),
            Err(ref err) => error!("{}", err)
        };
        self
    }
}
