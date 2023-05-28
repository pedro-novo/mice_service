use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
    Query,
}, get, post, put, delete, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{models::mouse::Mouse, repository::database::Database};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct QueryParams {
    pub brand: Option<String>,
    pub shape: Option<String>,
}

impl QueryParams {
    pub fn new(brand: Option<String>, shape: Option<String>) -> QueryParams {
        QueryParams { brand, shape }
    }
}


#[get("")]
pub async fn get_mice(db: Data<Database>, params: Query<QueryParams>) -> HttpResponse {
    let Query(params) = params;
    let mice = db.get_mice(params.brand, params.shape);
    HttpResponse::Ok().json(mice)
}

#[get("/{id}")]
pub async fn get_mouse_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let mouse = db.get_mouse_by_id(&id);
    match mouse {
        Some(mouse) => HttpResponse::Ok().json(mouse),
        None => HttpResponse::NotFound().body("Mouse Not Found!"),
    }
}

#[post("")]
pub async fn create_mouse(db: Data<Database>, new_mouse: Json<Mouse>) -> HttpResponse {
    let mouse = db.create_mouse(new_mouse.into_inner());
    match mouse {
        Ok(mouse) => HttpResponse::Ok().json(mouse),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/{id}")]
pub async fn update_mouse(db: Data<Database>, id: web::Path<String>, updated_mouse: Json<Mouse>) -> HttpResponse {
    let mouse = db.update_mouse_by_id(&id, updated_mouse.into_inner());
    match mouse {
        Some(mouse) => HttpResponse::Ok().json(mouse),
        None => HttpResponse::NotFound().body("Mouse Not Found!")
    }
}

#[delete("/{id}")]
pub async fn delete_mouse(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let mouse = db.delete_mouse_by_id(&id);
    match mouse {
        Some(mouse) => HttpResponse::Ok().json(mouse),
        None => HttpResponse::NotFound().body("Mouse Not Found!")
    }
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/mice")
            .service(get_mice)
            .service(get_mouse_by_id)
            .service(create_mouse)
            .service(update_mouse)
            .service(delete_mouse)
    );
}