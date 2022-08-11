#[macro_use]
extern crate diesel;
extern crate dotenv;

use warp::Filter;

mod errors;
pub mod group;
pub mod user;
use errors::handle_rejection;
use user::auth::{with_auth, Role};
use user::handlers::{login_handler, user_handler};

pub mod db;

#[tokio::main]
async fn main() {
    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::Admin))
        .and_then(user_handler);

    let routes = login_route.or(user_route).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
