pub use eventstore::OperationError;
use crate::Connection;
use std::{sync::Arc, fmt};

pub trait ESResultExt<T> {
    fn map_not_found(self) -> Result<Option<T>, OperationError>;
}

impl <T> ESResultExt<T> for Result<Option<T>, OperationError> {
    /// Map "not found"-type errors as None
    ///
    /// # Returns
    ///
    /// Some(x) if the result was found, None if there was a stream not found error, Err(e) if another error was encountered
    ///
    /// # Example
    ///
    /// ```
    /// use liquidity::db::{ESResultExt, OperationError};
    ///
    /// let some = Ok::<_, OperationError>(Some("test".to_string()));
    /// let none = Err::<Option<String>, _>(OperationError::StreamNotFound("asd".to_string()));
    /// let err = Err::<Option<String>, _>(OperationError::AccessDenied("asd".to_string()));
    ///
    /// assert_eq!(some.map_not_found().unwrap(), Some("test".to_string()));
    /// assert_eq!(none.map_not_found().unwrap(), None);
    /// assert!(err.map_not_found().is_err());
    /// ```
    fn map_not_found(self) -> Result<Option<T>, OperationError> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => {
                match e {
                    OperationError::StreamNotFound(_) | OperationError::StreamDeleted(_) => Ok(None),
                    _ => Err(e)
                }
            }
        }
    }
}

/// This is stupid and only for tracing
pub struct StupidConnectionWrapper(pub Arc<Connection>);
impl fmt::Debug for StupidConnectionWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "eventstore connection")
    }
}