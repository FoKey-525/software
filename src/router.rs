use actix_web::{web, HttpResponse, Responder, Scope};
use crate::controllers::{
    save_wallpaper::save_wallpaper,
    change_wallpaper::change_wallpaper,
    reset_wallpaper::reset_wallpaper,
    update_app::update_app,
};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("api")
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .route("/", web::get().to(index))
        .route("/save_wallpaper", web::post().to(save_wallpaper))
        .route("/change_wallpaper", web::get().to(change_wallpaper))
        .route("/reset_wallpaper", web::get().to(reset_wallpaper))
        .route("/update_app", web::get().to(update_app))
}

