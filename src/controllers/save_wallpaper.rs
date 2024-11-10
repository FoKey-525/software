use actix_web::{web, Result, Error};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
  id: String,
}

pub async fn save_wallpaper(info: web::Json<Info>) -> Result<String, Error> {
  Ok(format!("Welcome {}!", info.id))
}

