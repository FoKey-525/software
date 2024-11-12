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
  match info.function {
    0 => { save_wallpaper::save().await.expect("Error save_wallpaper in router"); },
    1 => { change_wallpaper::change().await.expect("Error change_wallpaper in router"); },
    2 => { reset_wallpaper::reset().await.expect("Error reset_wallpaper in router"); },
    3 => { update_app::update().await.expect("Error update_app in router"); },
    _ => todo!(),  
  }  
}

pub async fn api_post(info: web::Json<Info>) -> Result<String, Error> {
  distribution(&info).await;
  Ok("Request post ok".to_string())
}

pub async fn api_get(info: web::Json<Info>) -> Result<String, Error> {
  distribution(&info).await;
  Ok("request get ok".to_string())
}

pub fn api_scope() -> Scope {
  web::scope("/api")
    .route("/post", web::post().to(api_post))
    .route("/get", web::get().to(api_get))
}

