use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Galaxy {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StarSystem {
    pub id: i32,
    pub galaxy_id: i32,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub star_type: StarType,
    pub star_mass: f64,
    pub star_radius: f64,
    pub star_temperature: f64,
    pub star_luminosity: f64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "star_type", rename_all = "snake_case")]
pub enum StarType {
    MainSequence,
    RedGiant,
    WhiteDwarf,
    Neutron,
    BlackHole,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "celestial_body_type", rename_all = "lowercase")]
pub enum CelestialBodyType {
    Planet,
    Moon,
    Asteroid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CelestialBody {
    pub id: i32,
    pub system_id: i32, // Changed to i32
    pub name: String,
    pub body_type: CelestialBodyType,
    pub orbit_distance: f64,
    pub size: f64,
    pub mass: f64,
    pub temperature: f64,
    pub atmosphere: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CelestialBodyResource {
    pub body_id: i32,
    pub resource_id: i32,
    pub abundance: f64,
}