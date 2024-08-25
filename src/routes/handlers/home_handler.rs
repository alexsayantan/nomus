use actix_web::{get, Responder};

use crate::utils::response;


#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    response::ApiResponse::new(200, format!("Server Status Healthy"))
}
