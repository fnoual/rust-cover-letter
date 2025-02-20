use actix_web::{middleware as actix_middleware, web, App, HttpServer};
mod routes;
mod middleware;
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // .wrap(actix_middleware::Logger::default()) // Utilisation du renommage
            .wrap(actix_middleware::Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .configure(routes::config)
            .default_service(web::route().to(middleware::error_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
