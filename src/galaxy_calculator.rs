use crate::models::galaxy::{StarSystem, CelestialBody};

const LIGHT_SPEED: f64 = 299792.458;  // km/s
const WARP_FACTOR: f64 = 10.0;  // Arbitrary warp speed multiplier

pub fn calculate_distance(from: &StarSystem, to: &StarSystem) -> f64 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn calculate_travel_time(distance: f64, is_warp: bool) -> f64 {
    let speed = if is_warp {
        LIGHT_SPEED * WARP_FACTOR
    } else {
        LIGHT_SPEED * 0.1  // Sublight travel at 10% light speed
    };
    distance / speed  // Time in seconds
}

pub fn calculate_orbit_time(body: &CelestialBody) -> f64 {
    // Simplified orbit time calculation
    // In reality, this would depend on the star's mass and the body's orbit distance
    body.orbit_distance.powf(1.5) * 365.25 * 24.0 * 3600.0  // Time in seconds
}