mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use handlers::{create::create_item, read::{get_items, get_item}, update::update_item, delete::delete_item};
use models::Item;

struct AppState {
    items: Mutex<Vec<Item>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/items")
                    .route("", web::post().to(create_item))
                    .route("", web::get().to(get_items))
                    .route("/{id}", web::get().to(get_item))
                    .route("/{id}", web::put().to(update_item))
                    .route("/{id}", web::delete().to(delete_item)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
