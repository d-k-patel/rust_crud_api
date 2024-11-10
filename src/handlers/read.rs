use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::AppState;

pub async fn get_items(state: web::Data<AppState>) -> impl Responder {
    let items = state.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

pub async fn get_item(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let items = state.items.lock().unwrap();
    if let Some(item) = items.iter().find(|&x| x.id == *id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}
