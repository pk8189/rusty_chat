use super::models::users;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct APIUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
