use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::{CreateUpdateItem, Item};
use crate::AppState;

pub async fn create_item(state: web::Data<AppState>, item: web::Json<CreateUpdateItem>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    let new_item = Item {
        id: Uuid::new_v4(),
        name: item.name.clone(),
        description: item.description.clone(),
    };
    items.push(new_item.clone());
    HttpResponse::Created().json(new_item)
}
