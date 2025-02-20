use actix_web::{post, web, HttpResponse, Responder};
use crate::models::{JobApplication, CoverLetterResponse};
use chatgpt::prelude::*;
use dotenv::dotenv;
use std::env;

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
