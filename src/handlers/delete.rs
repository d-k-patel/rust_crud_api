use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::AppState;

pub async fn delete_item(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|x| x.id == *id) {
        items.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}
