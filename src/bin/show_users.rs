extern crate diesel;
extern crate rusty_chat;

use self::models::*;
use diesel::prelude::*;
use rusty_chat::*;

fn main() {
    use self::schema::users::dsl::*;

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
