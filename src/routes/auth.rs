use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::User;
use crate::db::DbPool;
use jsonwebtoken::{encode, EncodingKey, Header};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use std::env;
use crate::schema::users;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[post("/login")]
async fn login(pool: web::Data<DbPool>, creds: web::Json<LoginRequest>) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let user = users::table
        .filter(users::email.eq(&creds.email))
        .first::<User>(conn)
        .optional()
        .expect("Failed to query user");

    if let Some(user) = user {
        if verify(&creds.password, &user.password_hash).unwrap() {
            let claims = Claims {
                sub: user.id.to_string(),
                exp: 10000000000,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
            )
                .unwrap();

            return HttpResponse::Ok().json(serde_json::json!({ "token": token }));
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
