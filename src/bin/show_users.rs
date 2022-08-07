extern crate diesel;

use diesel::prelude::*;
use rusty_chat::db::establish_connection;
use rusty_chat::user::models::User;

#[allow(dead_code)]
fn main() {
    use rusty_chat::user::models::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("-----------");
        println!("{}", user.username);
    }
}
