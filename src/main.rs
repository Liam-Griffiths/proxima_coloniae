use actix_web::{web, App, HttpServer};
use actix_files as fs;
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod models;
mod handlers;
mod jwt;
mod services;
mod galaxy_generator;
mod galaxy_calculator;

use handlers::auth::{register, login, account};
use handlers::galaxy::{get_galaxy, get_star_systems, get_celestial_bodies};
use services::galaxy::GalaxyService;

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

    let galaxy_service = web::Data::new(GalaxyService::new(pool.clone()));

    // Initialize the galaxy
    match galaxy_service.initialize_galaxy().await {
        Ok(_) => println!("Galaxy initialized successfully"),
        Err(e) => eprintln!("Failed to initialize galaxy: {}", e),
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(galaxy_service.clone())
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/account").route(web::get().to(account)))
            .service(web::resource("/galaxy").route(web::get().to(get_galaxy)))
            .service(web::resource("/systems").route(web::get().to(get_star_systems)))
            .service(web::resource("/system/{id}/bodies").route(web::get().to(get_celestial_bodies)))
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}