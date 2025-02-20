use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{User, NewUser};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust API!")
}

#[get("/test")]
async fn get_user() -> impl Responder {
    let user = User {
        id: 1,
        username: "johndoe".to_string(),
        email: "johndoe@example.com".to_string(),
    };

    HttpResponse::Ok().json(user)
}

#[post("/user")]
async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    let new_user = User {
        id: 42,
        username: user.username.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(new_user)
}
