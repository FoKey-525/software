use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub username: String,
}

pub async fn save_wallpaper(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

