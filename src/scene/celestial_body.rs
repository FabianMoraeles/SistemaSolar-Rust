use crate::math::{Vec3, Mat4, create_transform_matrix};

pub struct CelestialBody {
    pub name: String,
    pub radius: f32,           // escala del planeta
    pub orbit_radius: f32,     // distancia al sol
    pub orbit_speed: f32,      // velocidad angular (rotación orbital)
    pub rotation_speed: f32,   // velocidad angular (rotación propia)
    pub orbit_angle: f32,      // estado actual de la órbita
    pub self_rotation: f32,    // estado actual de la rotación propia
    pub color: u32,            // color del planeta
}

impl CelestialBody {
    pub fn new(name: &str, radius: f32, orbit_radius: f32, orbit_speed: f32, rotation_speed: f32, color: u32) -> Self {
        Self {
            name: name.to_string(),
            radius,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            orbit_angle: 0.0,
            self_rotation: 0.0,
            color,
        }
    }

    /// Actualiza órbita + rotación interna
    pub fn update(&mut self, dt: f32) {
        self.orbit_angle += self.orbit_speed * dt;
        self.self_rotation += self.rotation_speed * dt;
    }

    /// Retorna la posición en el espacio 3D
    pub fn position(&self) -> Vec3 {
        Vec3::new(
            self.orbit_radius * self.orbit_angle.cos(),
            0.0,
            self.orbit_radius * self.orbit_angle.sin(),
        )
    }

    /// Retorna la matriz de modelo para el planeta
    pub fn model_matrix(&self) -> Mat4 {
        let pos = self.position();
        let rot = Vec3::new(0.0, self.self_rotation, 0.0);
        let scale = Vec3::new(self.radius, self.radius, self.radius);
        create_transform_matrix(pos, rot, scale)
    }
}
