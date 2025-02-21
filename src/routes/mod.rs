use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::validate_token;
use crate::routes::user::{hello, get_profile, get_user, create_user, register_user};

pub mod auth;
pub mod job;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user)
        .service(create_user)
        .service(job::generate_cover_letter)
        .service(auth::login)
        .service(register_user)
        .service(web::scope("/users")
            .wrap(HttpAuthentication::bearer(validate_token))
            .service(get_profile)
        )
        .service(hello);
}
