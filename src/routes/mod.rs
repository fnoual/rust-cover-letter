use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::validate_token;
use crate::routes::user::register_user;

pub mod auth;
pub mod job;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::get_user)
        .service(user::create_user)
        .service(job::generate_cover_letter)
        .service(auth::login)
        .service(register_user)
        .service(web::scope("/users").wrap(HttpAuthentication::bearer(validate_token))
        .service(user::get_profile))
        .service(user::hello);
}
