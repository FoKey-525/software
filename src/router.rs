use actix_web::{web, Scope, Result, Error};
use serde::Deserialize;

use crate::controllers::save_wallpaper;
use crate::controllers::change_wallpaper;
use crate::controllers::reset_wallpaper;
use crate::controllers::update_app;

#[derive(Deserialize)]
pub struct Info {
  id: i32,
  function: i32,
}

pub async fn distribution(info: &Info) {
  if info.function == 0 {
    let _ = save_wallpaper::save().await;
  } else if info.function == 1 {
    let _ = change_wallpaper::change().await; 
  } else if info.function == 2 {
    let _ = reset_wallpaper::reset().await; 
  } else if info.function == 3 {
    let _ = update_app::update().await;
  }
}

pub async fn api_post(info: web::Json<Info>) -> Result<String, Error> {
  distribution(&info).await;
  Ok("Super".to_string())
}

pub async fn api_get(info: web::Json<Info>) -> Result<String, Error> {
  distribution(&info).await;
  Ok("Super".to_string())
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .route("/post", web::post().to(api_post))
        .route("/get", web::get().to(api_get))
}

