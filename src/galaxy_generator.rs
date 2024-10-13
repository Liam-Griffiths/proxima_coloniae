use crate::models::galaxy::{Galaxy, StarSystem, StarType, CelestialBody, CelestialBodyType, CelestialBodyResource};
use rand::Rng;

pub fn generate_galaxy(name: String) -> (Galaxy, Vec<StarSystem>, Vec<CelestialBody>, Vec<CelestialBodyResource>) {
    let galaxy = Galaxy { id: 1, name };
    let mut systems = Vec::new();
    let mut bodies = Vec::new();
    let mut body_resources = Vec::new();

    // Create central system
    systems.push(generate_star_system(1, galaxy.id, "Central System".to_string(), 0.0, 0.0, 0.0));

    // Generate other systems in a sphere
    let mut rng = rand::thread_rng();
    for i in 1..10 {
        let (x, y, z) = generate_sphere_point(&mut rng);
        systems.push(generate_star_system(i + 1, galaxy.id, format!("System {}", i + 1), x, y, z));
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

fn generate_star_system(id: i32, galaxy_id: i32, name: String, x: f64, y: f64, z: f64) -> StarSystem {
    let mut rng = rand::thread_rng();
    let star_type = match rng.gen_range(0..100) {
        0..=70 => StarType::MainSequence,
        71..=85 => StarType::RedGiant,
        86..=95 => StarType::WhiteDwarf,
        96..=98 => StarType::Neutron,
        _ => StarType::BlackHole,
    };

    let (mass, radius, temperature, luminosity) = match star_type {
        StarType::MainSequence => (rng.gen_range(0.1..50.0), rng.gen_range(0.1..50.0), rng.gen_range(2000.0..30000.0), rng.gen_range(0.0001..1000000.0)),
        StarType::RedGiant => (rng.gen_range(0.5..10.0), rng.gen_range(10.0..1000.0), rng.gen_range(3000.0..5000.0), rng.gen_range(100.0..10000.0)),
        StarType::WhiteDwarf => (rng.gen_range(0.17..1.33), rng.gen_range(0.008..0.02), rng.gen_range(4000.0..40000.0), rng.gen_range(0.0001..0.1)),
        StarType::Neutron => (rng.gen_range(1.4..3.0), rng.gen_range(1e-5..2e-5), rng.gen_range(1e5..1e6), rng.gen_range(0.1..1.0)),
        StarType::BlackHole => (rng.gen_range(3.0..100.0), 0.0, 0.0, 0.0),
    };

    StarSystem {
        id,
        galaxy_id,
        name,
        x, y, z,
        star_type,
        star_mass: mass,
        star_radius: radius,
        star_temperature: temperature,
        star_luminosity: luminosity,
    }
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
