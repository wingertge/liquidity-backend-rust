use crate::auth::JWTError;
use liquidity::context::User;

fn all_perms() -> Vec<String> {
    vec!["create:election", "update:election", "view:election"]
        .into_iter()
        .map(ToString::to_string)
        .collect()
}

pub fn create_test_user() -> Option<Result<User, JWTError>> {
    let user = User {
        id: "test_user_id".to_string(),
        permissions: all_perms()
    };
    Some(Ok(user))
}
