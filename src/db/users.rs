use super::{DbConnection, models::{NewUser, User}, schema::users};
use diesel::RunQueryDsl;
use failure::{Error, ResultExt};

pub fn create_user<'a>(id: String, username: String, conn: &DbConnection) -> Result<User, Error> {
    let new_user = NewUser {
        id: id.as_str(),
        username: username.as_str()
    };

    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .context("Failed to insert new user")?
    )
}