use actix_web::{web, Responder, Scope};
use crate::controllers::{biology::biology, geography::geography, physics::physics};

pub async fn index() -> impl Responder {
  "api"
}

pub fn api_scope() -> Scope {
  web::scope("/api")
    .route("/", web::get().to(index))
    .route("/geography", web::get().to(geography))
    .route("/biology", web::get().to(biology))
    .route("/physics", web::get().to(physics))
}
