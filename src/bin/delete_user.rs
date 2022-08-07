extern crate diesel;

use self::diesel::prelude::*;
use std::env::args;

use rusty_chat::db::establish_connection;

#[allow(dead_code)]
fn main() {
    use rusty_chat::user::models::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(users.filter(username.like(pattern)))
        .execute(&connection)
        .expect("Error deleting user");

    if num_deleted == 0 {
        println!("No user with name {}. No user deleted", target);
    } else {
        println!("Deleted {}", target);
    }
}
