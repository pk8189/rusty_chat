use diesel::prelude::*;
use diesel::PgConnection;
use warp::{reject, reply, Rejection, Reply};

use super::api_schemas::{LoginRequest, LoginResponse};
use super::auth::{create_jwt, Role};
use super::models::{users, NewUser, User};

use super::super::db;
use super::super::errors::Error;

pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, Error>;

pub async fn login_handler(body: LoginRequest) -> WebResult<impl Reply> {
    log::debug!("{:?}", body);
    use self::users::dsl::*;
    let connection = db::establish_connection();
    let selected_user = users
        .filter(username.eq(body.username))
        .filter(password.eq(body.password))
        .first::<User>(&connection);

    match selected_user {
        Ok(selected_user) => {
            let token =
                create_jwt(&selected_user.id, &Role::Admin).map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        Err(_) => Err(reject::custom(Error::WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    let new_user = NewUser {
        username: username,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user!")
}
