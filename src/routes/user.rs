use actix_web::{get, post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use crate::db::DbPool;
use crate::models::{User, NewUser};
use diesel::prelude::*;
use crate::schema::users;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust API!")
}

#[get("/test")]
async fn get_user() -> impl Responder {
    let hashed_password = hash("fnoual123", DEFAULT_COST).unwrap();

    let user = User {
        id: 1,
        username: "johndoe".to_string(),
        email: "johndoe@example.com".to_string(),
        password_hash:hashed_password
    };

    HttpResponse::Ok().json(user)
}

#[post("/user")]
async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    let hashed_password = hash("fnoual123", DEFAULT_COST).unwrap();

    let new_user = User {
        id: 42,
        username: user.username.clone(),
        email: user.email.clone(),
        password_hash:hashed_password
    };

    HttpResponse::Created().json(new_user)
}

#[post("/register")]
async fn register_user(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let hashed_password = hash(&user.password_hash, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: user.username.clone(),
        email: user.email.clone(),
        password_hash: hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Failed to insert user");

    HttpResponse::Created().body("User registered successfully")
}

#[get("/profile")]
async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("User profile")
}