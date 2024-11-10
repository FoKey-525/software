// main.rs
use actix_web::{web, App, HttpServer};
mod controllers; // Подключаем модуль controllers

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(controllers::save_wallpaper::save_wallpaper)) // Правильный путь
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

