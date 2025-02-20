use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
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
