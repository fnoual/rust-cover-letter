use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct JobApplication {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
    pub job_name: String,
}

#[derive(Serialize)]
pub struct CoverLetterResponse {
    pub cover_letter: String,
}
