use actix_web::web;

mod user;
mod job;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::get_user)
       .service(user::create_user)
       .service(job::generate_cover_letter)
       .service(user::hello);
}
