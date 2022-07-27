#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;

use auth::{with_auth, Role};
use error::Error::*;
use serde::{Deserialize, Serialize};
use warp::{reject, reply, Filter, Rejection, Reply};

mod auth;
mod error;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

pub mod models;
pub mod schema;

mod db;

#[derive(Clone)]
pub struct APIUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

use schema::*;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[tokio::main]
async fn main() {
    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::Admin))
        .and_then(user_handler);

    let routes = login_route.or(user_route).recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub async fn login_handler(body: LoginRequest) -> WebResult<impl Reply> {
    log::debug!("{:?}", body);
    use self::schema::users::dsl::*;
    let connection = db::establish_connection();
    let selected_user = users
        .filter(username.eq(body.username))
        .filter(password.eq(body.password))
        .first::<User>(&connection);

    match selected_user {
        Ok(selected_user) => {
            let token = auth::create_jwt(&selected_user.id, &Role::from_str("ADMIN"))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        Err(_) => Err(reject::custom(WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}
