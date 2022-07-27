extern crate diesel;
extern crate rusty_chat;

use self::diesel::prelude::*;
use self::rusty_chat::*;
use std::env::args;

fn main() {
    use rusty_chat::schema::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(users.filter(username.like(pattern)))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deleted {} users", num_deleted);
    println!("Deleted {}", target);
}
