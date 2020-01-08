use super::context::User;
use juniper::FieldError;

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
/// # use backend_rust::graphql::context::User;
/// # use backend_rust::graphql::permissions;
/// # use juniper::FieldError;
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
/// assert_eq!(Err(FieldError::from("You don't have permission to view elections")), invalid);
/// assert_eq!(Err(FieldError::from("Must be logged in to do that")), not_logged_in);
/// ```
pub fn check<'a>(key: &str, user: &Option<User>) -> Result<(), FieldError> {
    let has_permission = || -> Option<bool> { Some(user.as_ref()?.permissions.contains(&key.to_string())) };
    match has_permission() {
        None => Err("Must be logged in to do that".into()),
        Some(false) => Err("You don't have permission to view elections".into()),
        Some(true) => Ok(())
    }
}
