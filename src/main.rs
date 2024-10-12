use actix_web::{web, App, HttpServer};
use actix_files as fs;
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod models;
mod handlers;
mod jwt;

use handlers::auth::{register, login, account};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/account").route(web::get().to(account)))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}