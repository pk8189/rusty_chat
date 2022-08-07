extern crate diesel;

use rusty_chat::db::establish_connection;
use rusty_chat::user::handlers::create_user;
use std::io::stdin;

#[allow(dead_code)]
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
