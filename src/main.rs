use actix_web::{middleware,get, post, web, App, HttpServer, HttpRequest, Responder, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use chatgpt::prelude::*;
use dotenv::dotenv;
use std::env;


#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Middleware de capture d'erreur
async fn error_handler(req: HttpRequest) -> Result<HttpResponse> {
    let error_response = ErrorResponse {
        error: format!("Route '{}' not found", req.path()),
    };

    Ok(HttpResponse::NotFound().json(error_response))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust API!")
}

#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
    email: String,
}

#[get("/test")]
async fn get_user() -> impl Responder {
    let user = User {
        id: 1,
        username: "johndoe".to_string(),
        email: "johndoe@example.com".to_string(),
    };

    HttpResponse::Ok().json(user) // Retourne un JSON
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    email: String,
}

#[post("/user")]
async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    let new_user = User {
        id: 42, // ID simulé
        username: user.username.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(new_user) // Retourne l'utilisateur créé
}

#[derive(Deserialize)]
struct JobApplication {
    first_name: String,
    last_name: String,
    age: u32,
    job_name: String,
}

#[derive(Serialize)]
struct CoverLetterResponse {
    cover_letter: String,
}

#[post("/cover-letter")]
async fn generate_cover_letter(data: web::Json<JobApplication>) -> impl Responder {
    dotenv().ok();

    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => return HttpResponse::InternalServerError().body("Missing API Key"),
    };

    let client = match ChatGPT::new(api_key) {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to create ChatGPT client"),
    };

    let prompt = format!(
        "Rédige une lettre de motivation en français pour un poste de {}.
        Le candidat s'appelle {} {} et a {} ans.
        La lettre doit être formelle et bien rédigée.",
        data.job_name, data.first_name, data.last_name, data.age
    );

    match client.send_message(prompt).await {
        Ok(response) => HttpResponse::Ok().json(CoverLetterResponse {
            cover_letter: response.message().content.clone(),
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to generate cover letter"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) 
            .service(hello)
            .service(get_user)
            .service(create_user) // Ajout du POST
            .service(generate_cover_letter)
            .default_service(web::route().to(error_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
