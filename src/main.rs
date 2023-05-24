mod repository;
mod models;
mod api;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mouse_db = repository::database::Database::new();
    let app_data = web::Data::new(mouse_db);

    HttpServer::new(move || 
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .wrap(actix_web::middleware::Logger::default())
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
