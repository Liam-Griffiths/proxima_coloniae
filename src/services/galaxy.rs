use sqlx::PgPool;
use crate::models::galaxy::{Galaxy, StarSystem, CelestialBody, CelestialBodyType};

pub struct GalaxyService {
    pool: PgPool,
}

impl GalaxyService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn initialize_galaxy(&self) -> Result<Galaxy, sqlx::Error> {
        let existing_galaxy = sqlx::query!(
            "SELECT id, name FROM galaxies LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = existing_galaxy {
            Ok(Galaxy {
                id: row.id,
                name: row.name,
            })
        } else {
            // Galaxy doesn't exist, create it
            self.create_galaxy("Proxima Coloniae Galaxy".to_string()).await
        }
    }

    async fn create_galaxy(&self, name: String) -> Result<Galaxy, sqlx::Error> {
        let galaxy = sqlx::query!(
            "INSERT INTO galaxies (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Galaxy {
            id: galaxy.id,
            name: galaxy.name,
        })
    }

    pub async fn get_galaxy(&self) -> Result<Galaxy, sqlx::Error> {
        let galaxy = sqlx::query!(
            "SELECT id, name FROM galaxies LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Galaxy {
            id: galaxy.id,
            name: galaxy.name,
        })
    }

    pub async fn get_star_systems(&self) -> Result<Vec<StarSystem>, sqlx::Error> {
        let systems = sqlx::query!(
            "SELECT id, galaxy_id as \"galaxy_id!: i32\", name, x, y, z FROM star_systems"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(systems
            .into_iter()
            .map(|row| StarSystem {
                id: row.id,
                galaxy_id: row.galaxy_id,
                name: row.name,
                x: row.x,
                y: row.y,
                z: row.z,
            })
            .collect())
    }

    pub async fn get_celestial_bodies(&self, system_id: i32) -> Result<Vec<CelestialBody>, sqlx::Error> {
        let bodies = sqlx::query!(
            r#"
            SELECT
                id, system_id as "system_id!: i32", name,
                body_type as "body_type!: CelestialBodyType",
                orbit_distance, size, mass, temperature, atmosphere
            FROM celestial_bodies
            WHERE system_id = $1
            "#,
            system_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(bodies
            .into_iter()
            .map(|row| CelestialBody {
                id: row.id,
                system_id: row.system_id,
                name: row.name,
                body_type: row.body_type,
                orbit_distance: row.orbit_distance,
                size: row.size,
                mass: row.mass,
                temperature: row.temperature,
                atmosphere: row.atmosphere,
            })
            .collect())
    }

}
