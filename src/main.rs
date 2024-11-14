use actix_web::{App, HttpServer};

mod router;                

#[actix_web::main]         
pub async fn main() -> std::io::Result<()> { 
  let port = 4000;
  println!("Start server {}", port);

  HttpServer::new(|| {   
    App::new().service(router::api_scope())
  })
  .bind(("0.0.0.0", port))?
  .run()
  .await  
}

