use actix_web::{App, HttpServer};

mod router;                
mod controllers {
    pub mod save_wallpaper;     
    pub mod change_wallpaper;       
    pub mod reset_wallpaper;       
    pub mod update_app;
}

#[actix_web::main]         
async fn main() -> std::io::Result<()> { 
    HttpServer::new(|| {   
        App::new().service(router::api_scope())
    })
    .bind(("127.0.0.1", 4400))?
    .run()
    .await
}

