use actix_web;
use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::{Serialize};

use crate::api::mouse;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .configure(mouse::mouse::config)
        .service(healthcheck)
        .default_service(web::route().to(not_found))
    );
}