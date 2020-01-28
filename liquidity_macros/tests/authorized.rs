use liquidity::context::User;
use liquidity::{Context, Error};
use liquidity::permissions::PermissionError;
use liquidity_test_utils::connection::MockConnection;
use liquidity_macros::authorized;

#[derive(Debug)]
struct TestContext {
    user: Option<User>
}

impl Context<MockConnection> for TestContext {
    fn db(&self) -> MockConnection {
        unimplemented!()
    }

    fn user(&self) -> &Option<User> {
        &self.user
    }
}

#[authorized]
fn requires_authentication(context: TestContext) -> Result<i32, Error> {
    Ok(1)
}

#[authorized("pass:test")]
fn requires_authorization(context: TestContext) -> Result<i32, Error> {
    Ok(1)
}

#[test]
fn authentication_present() {
    let user = Some(User {
        id: "test".to_string(),
        permissions: vec![]
    });
    let ctx = TestContext { user };

    let result = requires_authentication(ctx);

    assert_eq!(result.unwrap(), 1);
}

#[test]
fn authentication_missing() {
    let ctx = TestContext { user: None };

    let result = requires_authentication(ctx);

    assert_eq!(format!("{}", result.unwrap_err()), format!("{}", PermissionError::NotLoggedIn))
}

#[test]
fn authorization_present() {
    let user = Some(User {
        id: "test".to_string(),
        permissions: vec!["pass:test".to_string()]
    });
    let ctx = TestContext { user };

    let result = requires_authorization(ctx);

    assert_eq!(result.unwrap(), 1);
}

#[test]
fn authorization_not_logged_in() {
    let ctx = TestContext { user: None };

    let result = requires_authorization(ctx);

    assert_eq!(format!("{}", result.unwrap_err()), format!("{}", PermissionError::NotLoggedIn));
}

#[test]
fn authorization_missing() {
    let user = Some(User {
        id: "test".to_string(),
        permissions: vec![]
    });
    let ctx = TestContext { user };

    let result = requires_authorization(ctx);

    assert_eq!(format!("{}", result.unwrap_err()), format!("{}", PermissionError::NotAllowed));
}