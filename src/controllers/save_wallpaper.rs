use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use actix_web::Error;

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
}

pub async fn save_wallpaper(info: web::Json<Info>) -> Result<String, actix_web::Error> {
    Ok(format!("{}", info.name))
}

