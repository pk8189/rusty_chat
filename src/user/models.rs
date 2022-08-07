// define the literal models here
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

// define the model interfaces here (structs)
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
