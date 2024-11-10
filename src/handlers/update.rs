use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::CreateUpdateItem;
use crate::AppState;

pub async fn update_item(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    item: web::Json<CreateUpdateItem>,
) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(existing_item) = items.iter_mut().find(|x| x.id == *id) {
        existing_item.name = item.name.clone();
        existing_item.description = item.description.clone();
        HttpResponse::Ok().json(existing_item.clone())
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}
