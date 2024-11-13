use actix_web::{web, Scope, Result, Error};
use serde::Deserialize;

use crate::controllers::save_wallpaper;
use crate::controllers::change_wallpaper;
use crate::controllers::reset_wallpaper;
use crate::controllers::update_app;

#[derive(Deserialize)]
pub struct Info {
  id: i32,
  function: i32
}

pub async fn api_post(info: web::Json<Info>) -> Result<String, Error> {
  match info.function {
    1 => { save_wallpaper::save().await.expect("Error save_wallpaper in router"); Ok("1".to_string())},
    2 => { change_wallpaper::change().await.expect("Error change_wallpaper in router"); Ok("2".to_string())},
    3 => { reset_wallpaper::reset().await.expect("Error reset_wallpaper in router");Ok("3".to_string()) },
    4 => { update_app::update().await.expect("Error update_app in router"); Ok("4".to_string())},
    _ => todo!(),  
  }
}

pub async fn api_get(info: web::Json<Info>) -> Result<String, Error> {
  let id = info.id as usize;
  let function = info.function as usize;
  let mut clients = [[false; 4]; 4];
  
  if id < clients.len() && function < clients[id].len() {
    clients[id][function] = true; 
    println!("True {} {}", id, function);
  }

  Ok("Ok".to_string())
}

pub fn api_scope() -> Scope {
  web::scope("/api")
    .route("/post", web::post().to(api_post))
    .route("/get", web::get().to(api_get))
}

