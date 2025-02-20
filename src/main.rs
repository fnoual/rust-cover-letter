use actix_web::{middleware as actix_middleware, web, App, HttpServer};
mod routes;
mod middleware;
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_middleware::Logger::default()) // Utilisation du renommage
            .configure(routes::config)
            .default_service(web::route().to(middleware::error_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
