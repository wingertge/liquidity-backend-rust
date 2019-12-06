use crate::context::User;

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
/// # use backend_rust::context::User;
/// # use backend_rust::permissions;
/// // Make a mock user
/// let user = Some(Box::new(User {
///     id: "".to_string(),
///     permissions: vec!["view:election".to_string()]
/// }));
///
/// let valid = permissions::check("view:election", &user);
/// let invalid = permissions::check("create:election", &user);
/// let not_logged_in = permissions::check("create:election", &None);
///
/// assert_eq!(Ok(true), valid);
/// assert_eq!(Err("You don't have permission to view elections"), invalid);
/// assert_eq!(Err("Must be logged in to do that"), not_logged_in);
/// ```
pub fn check<'a>(key: &str, user: &Option<Box<User>>) -> Result<bool, &'a str> {
    if !user.as_ref()
        .ok_or("Must be logged in to do that")?
        .permissions.contains(&key.to_string()) {
        return Err("You don't have permission to view elections")
    }
    Ok(true)
}
