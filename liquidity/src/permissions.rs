use super::context::User;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum PermissionError {
    NotLoggedIn,
    NotAllowed
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PermissionError: {:?}", self)
    }
}

impl Error for PermissionError {
    fn description(&self) -> &str {
        match self {
            PermissionError::NotLoggedIn => "Must be logged in to do that",
            PermissionError::NotAllowed => "You don't have permission to do that"
        }
    }
}

/// Check the user's permissions to ensure they are allowed to use the API function
/// # Arguments
///
/// * `key` - A string that holds the permission key required for access to this API
/// * `user` - The user object that holds the permissions
///
/// # Returns
///
/// True if the user has permission, failure::Error if the user doesn't have permission or isn't logged in
///
/// # Example
///
/// ```
/// # use liquidity::context::User;
/// use liquidity::{permissions::{self, PermissionError}};
///
/// // Make a mock user
/// let user = Some(User {
///     id: "".to_string(),
///     permissions: vec!["view:election".to_string()]
/// });
///
/// let valid = permissions::check("view:election", &user);
/// let invalid = permissions::check("create:election", &user);
/// let not_logged_in = permissions::check("create:election", &None);
///
/// assert_eq!(Ok(()), valid);
/// assert_eq!(Err(PermissionError::NotAllowed), invalid);
/// assert_eq!(Err(PermissionError::NotLoggedIn), not_logged_in);
/// ```
pub fn check<'a>(key: &str, user: &Option<User>) -> Result<(), PermissionError> {
    let has_permission = || -> Option<bool> { Some(user.as_ref()?.permissions.contains(&key.to_string())) };
    match has_permission() {
        None => Err(PermissionError::NotLoggedIn),
        Some(false) => Err(PermissionError::NotAllowed),
        Some(true) => Ok(())
    }
}
