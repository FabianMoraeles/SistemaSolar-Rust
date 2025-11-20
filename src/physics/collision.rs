use crate::math::Vec3;
use crate::scene::solar_system::SolarSystem;
use crate::scene::celestial_body::CelestialBody;

/// Corrige la posición de la cámara si está demasiado cerca de algún cuerpo
pub fn resolve_camera_collisions(system: &SolarSystem, camera_pos: &mut Vec3, margin_factor: f32) {
    handle_body(&system.sun, camera_pos, margin_factor);

    for planet in &system.planets {
        handle_body(planet, camera_pos, margin_factor);
    }
}

fn handle_body(body: &CelestialBody, camera_pos: &mut Vec3, margin_factor: f32) {
    let center = body.position();
    let min_dist = body.radius * margin_factor;

    let offset = *camera_pos - center;
    let dist = offset.length();

    if dist < min_dist && dist > 0.0001 {
        let dir = offset / dist;
        *camera_pos = center + dir * min_dist;
    }
}
