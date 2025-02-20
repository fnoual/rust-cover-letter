use actix_web::{HttpRequest, HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn error_handler(req: HttpRequest) -> Result<HttpResponse> {
    let error_response = ErrorResponse {
        error: format!("Route '{}' not found", req.path()),
    };

    Ok(HttpResponse::NotFound().json(error_response))
}
