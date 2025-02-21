use actix_web::{middleware as actix_middleware, web, App, HttpServer};
mod routes;
pub mod middleware;
mod models;
mod handlers;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let pool = db::init_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .configure(routes::config)
            .default_service(web::route().to(middleware::error_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}