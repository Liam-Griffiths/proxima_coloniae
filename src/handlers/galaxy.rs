use actix_web::{web, HttpResponse, Error};
use crate::services::galaxy::GalaxyService;
// use crate::jwt::{verify_token, get_token_from_headers};

pub async fn get_galaxy(
    galaxy_service: web::Data<GalaxyService>,
) -> Result<HttpResponse, Error> {
    match galaxy_service.get_galaxy().await {
        Ok(galaxy) => Ok(HttpResponse::Ok().json(galaxy)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to retrieve galaxy")),
    }
}

pub async fn get_star_systems(
    galaxy_service: web::Data<GalaxyService>,
) -> Result<HttpResponse, Error> {
    match galaxy_service.get_star_systems().await {
        Ok(systems) => Ok(HttpResponse::Ok().json(systems)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to retrieve star systems")),
    }
}

pub async fn get_celestial_bodies(
    galaxy_service: web::Data<GalaxyService>,
    system_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match galaxy_service.get_celestial_bodies(system_id.into_inner()).await {
        Ok(bodies) => Ok(HttpResponse::Ok().json(bodies)),
        Err(_) => Ok(HttpResponse::NotFound().json("Celestial bodies not found")),
    }
}