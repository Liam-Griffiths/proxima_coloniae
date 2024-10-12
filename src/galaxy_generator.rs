use crate::models::galaxy::{Galaxy, StarSystem, CelestialBody, CelestialBodyType, CelestialBodyResource};
use rand::Rng;

pub fn generate_galaxy(name: String) -> (Galaxy, Vec<StarSystem>, Vec<CelestialBody>, Vec<CelestialBodyResource>) {
    let galaxy = Galaxy { id: 1, name };
    let mut systems = Vec::new();
    let mut bodies = Vec::new();
    let mut body_resources = Vec::new();

    // Create central system
    systems.push(StarSystem {
        id: 1,
        galaxy_id: galaxy.id,
        name: "Central System".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    // Generate other systems in a sphere
    let mut rng = rand::thread_rng();
    for i in 1..10 {
        let (x, y, z) = generate_sphere_point(&mut rng);
        systems.push(StarSystem {
            id: i + 1,
            galaxy_id: galaxy.id,
            name: format!("System {}", i + 1),
            x, y, z,
        });
    }

    // Generate celestial bodies for each system
    for system in &systems {
        let num_bodies = rng.gen_range(3..8);
        for j in 0..num_bodies {
            let body = generate_celestial_body(system.id, j + 1, &mut rng);
            bodies.push(body);
        }
    }

    // Generate resources for each body
    for body in &bodies {
        let num_resources = rng.gen_range(1..5);
        for _ in 0..num_resources {
            body_resources.push(CelestialBodyResource {
                body_id: body.id,
                resource_id: rng.gen_range(1..11),  // Assuming 10 different resources
                abundance: rng.gen_range(0.0..1.0),
            });
        }
    }

    (galaxy, systems, bodies, body_resources)
}

fn generate_celestial_body(system_id: i32, body_number: i32, rng: &mut impl Rng) -> CelestialBody {
    CelestialBody {
        id: body_number,
        system_id,
        name: format!("Body {}", body_number),
        body_type: match rng.gen_range(0..10) {
            0..=6 => CelestialBodyType::Planet,
            7..=8 => CelestialBodyType::Moon,
            _ => CelestialBodyType::Asteroid,
        },
        orbit_distance: rng.gen_range(0.1..10.0),
        size: rng.gen_range(100.0..100000.0),
        mass: rng.gen_range(1e20..1e27),
        temperature: rng.gen_range(50.0..500.0),
        atmosphere: if rng.gen_bool(0.5) {
            Some("Generic atmosphere".to_string())
        } else {
            None
        },
    }
}

fn generate_sphere_point(rng: &mut impl Rng) -> (f64, f64, f64) {
    let theta = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    let phi = rng.gen_range(0.0..std::f64::consts::PI);
    let r = 1.0;  // Uniform sphere of radius 1

    let x = r * phi.sin() * theta.cos();
    let y = r * phi.sin() * theta.sin();
    let z = r * phi.cos();

    (x, y, z)
}
