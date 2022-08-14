extern crate diesel;
use self::diesel::prelude::*;

use rusty_chat::db::establish_connection;
use rusty_chat::group::handlers::{add_members_to_group, create_group};
use rusty_chat::user::models::users::dsl::*;

use std::io::stdin;

#[allow(dead_code)]
fn main() {
    let connection = establish_connection();

    println!("What is your username");
    let mut pattern = String::new();
    stdin().read_line(&mut pattern).unwrap();
    let pattern = &pattern[..(pattern.len() - 1)]; // Drop the newline character
    let user_id = users
        .limit(1)
        .filter(username.like(pattern))
        .select(id)
        .first::<i32>(&connection)
        .expect("Error loading user");

    println!("What would you like your new group name to be?");
    let mut group_name = String::new();
    stdin().read_line(&mut group_name).unwrap();
    let group_name = &group_name[..(group_name.len() - 1)]; // Drop the newline character

    println!("What user id you like to add to your group?");
    let mut new_user_id = String::new();
    stdin().read_line(&mut new_user_id).unwrap();
    let new_user_id = &new_user_id[..(new_user_id).len() - 1]; // Drop the newline character
    let new_user_id_int = new_user_id.parse::<i32>().unwrap();
    let member_ids: Vec<i32> = [user_id, new_user_id_int].to_vec();

    let group = create_group(&connection, &user_id, group_name);
    let group_id = &group.id;
    let _members_added = add_members_to_group(&connection, group_id, member_ids);
    println!(
        "Successfully created group {} with ID {} ",
        group.name, group.id
    );
}
