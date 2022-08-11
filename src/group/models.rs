table! {
    groups (id) {
        id -> Int4,
        owner_id -> Int4,
        name -> VarChar,
    }
}

table! {
    group_users (id) {
        id -> Int4,
        group_id -> Int4,
        member_id -> Int4,
    }
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup<'a> {
    pub owner_id: &'a i32,
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct Group {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "group_users"]
pub struct NewGroupUser<'a> {
    pub(crate) group_id: &'a i32,
    pub(crate) member_id: &'a i32,
}
