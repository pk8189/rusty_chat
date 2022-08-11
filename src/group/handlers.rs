use super::models::{group_users, groups, Group, NewGroup, NewGroupUser};
use diesel::prelude::*;
use diesel::PgConnection;

pub fn create_group<'a>(conn: &PgConnection, owner_id: &'a i32, name: &'a str) -> Group {
    let new_group = NewGroup {
        owner_id: owner_id,
        name: name,
    };

    diesel::insert_into(groups::table)
        .values(&new_group)
        .get_result(conn)
        .expect("Error saving new group!")
}

pub type NewGroupUsers<'a> = Vec<NewGroupUser<'a>>;

pub fn add_members_to_group<'a>(conn: &PgConnection, group_id: &'a i32, member_ids: Vec<i32>) {
    // no return type here because bulk insert is weird. consider doing something with res for handling
    let new_group_users: NewGroupUsers = member_ids
        .iter()
        .map(|member_id| NewGroupUser {
            group_id: group_id,
            member_id: member_id,
        })
        .collect();
    let _res = diesel::insert_into(group_users::table)
        .values(&new_group_users)
        .execute(conn)
        .expect("Error saving new group members!");
}
