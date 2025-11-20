use minifb::Window;
use crate::math::{Vec3, Mat4};

pub struct FreeCamera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub mouse_sensitivity: f32,
}

impl FreeCamera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            speed: 12.0,
            mouse_sensitivity: 0.002,
        }
    }

    // ----------------------------
    // DIRECCIONES
    // ----------------------------
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            -self.pitch.sin(),
            self.yaw.cos() * self.pitch.cos(),
        ).normalize()
    }

    pub fn right(&self) -> Vec3 {
        Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin()).normalize()
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(&self.forward()).normalize()
    }

    // ----------------------------
    // MOVIMIENTOS (compatibilidad con InputController)
    // ----------------------------
    pub fn move_forward(&mut self, amount: f32) {
        self.position = self.position + self.forward() * amount;
    }

    pub fn move_backward(&mut self, amount: f32) {
        self.position = self.position - self.forward() * amount;
    }

    pub fn move_left(&mut self, amount: f32) {
        self.position = self.position - self.right() * amount;
    }

    pub fn move_right(&mut self, amount: f32) {
        self.position = self.position + self.right() * amount;
    }

    pub fn move_up(&mut self, amount: f32) {
        self.position.y += amount;
    }

    pub fn move_down(&mut self, amount: f32) {
        self.position.y -= amount;
    }

    // ----------------------------
    // VIEW MATRIX
    // ----------------------------
    pub fn view_matrix(&self) -> Mat4 {
        let target = self.position + self.forward();
        Mat4::look_at(self.position, target, Vec3::unit_y())
    }
}
