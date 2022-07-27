extern crate diesel;
extern crate rusty_chat;

use diesel::prelude::*;
use rusty_chat::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your username to be?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character
    println!("What would you like your password to be?");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len() - 1)]; // Drop the newline character

    let user = create_user(&connection, username, password);
    println!(
        "Successfully created user {} with ID {} ",
        user.username, user.id
    );
}

use self::models::{NewUser, User};

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        username: username,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user!")
}
