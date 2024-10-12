use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod routes;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    println!("Starting Proxima Coloniae server...");

    HttpServer::new(|| {
        App::new()
            // Add routes here as we develop them
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
