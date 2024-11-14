use actix_web::{web, Scope, Result, Error};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct Info {
  id: i32,
  function: i32,
}

type Clients = Mutex<[[bool; 4]; 4]>;

pub async fn api_post(info: web::Json<Info>, clients: web::Data<Clients>) -> Result<String, Error> {
  let mut clients = clients.lock().unwrap();
  let id = info.id as usize;
  let function = info.function as usize;
  let current_value = clients[id][function].to_string();

  clients[id][function] = false;

  Ok(current_value)
}

pub async fn api_get(info: web::Json<Info>, clients: web::Data<Clients>) -> Result<String, Error> {
  let id = info.id as usize;
  let function = info.function as usize;
  let mut clients = clients.lock().unwrap();

  if id < clients.len() && function < clients[id].len() {
    clients[id][function] = true;
  }

  Ok("Ok".to_string())
}

pub fn api_scope() -> Scope {
  web::scope("/api")
    .app_data(web::Data::new(Mutex::new([[false; 4]; 4])))
    .route("/post", web::post().to(api_post))
    .route("/get", web::get().to(api_get))
}

