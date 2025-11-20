use crate::math::{Vec3, Mat4};

pub struct SpaceShip {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub scale: f32,
    pub mesh: crate::models::Mesh,
}

impl SpaceShip {
    pub fn new(mesh: crate::models::Mesh) -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            scale: 0.4, // ⚠️ tamaño reducido para que no tape todo
            mesh,
        }
    }

    pub fn update_from_camera(&mut self, camera: &crate::camera::FreeCamera) {
        // La nave rota EXACTAMENTE como la cámara
        self.yaw = camera.yaw;
        self.pitch = camera.pitch;

        // La nave va un poco enfrente de la cámara
        self.position = camera.position + camera.forward() * 2.5;
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::translation(self.position.x, self.position.y, self.position.z)
            * Mat4::rotation_y(self.yaw)
            * Mat4::rotation_x(self.pitch)
            * Mat4::scale(self.scale, self.scale, self.scale)
    }
}
