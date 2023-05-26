mod repository;
mod models;
mod api;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mouse_db = repository::database::Database::new();
    let app_data = web::Data::new(mouse_db);
    let fe_url = std::env::var("FE_APP_URL").expect("FE_APP_URL must be set");

    HttpServer::new(move || 
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin(&fe_url)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
