use minifb::{Window, Key, MouseMode};
use crate::camera::freecam::FreeCamera;

pub struct InputController {
    last_mouse_pos: Option<(f32, f32)>,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            last_mouse_pos: None,
        }
    }

    pub fn update(&mut self, window: &Window, camera: &mut FreeCamera, dt: f32) {
        // ============= MOVIMIENTO =============
        let mut speed_mult = 1.0;

        if window.is_key_down(Key::LeftShift) {
            speed_mult = 3.0;
        }

        if window.is_key_down(Key::W) {
            camera.move_forward(dt * speed_mult);
        }
        if window.is_key_down(Key::S) {
            camera.move_backward(dt * speed_mult);
        }
        if window.is_key_down(Key::A) {
            camera.move_left(dt * speed_mult);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(dt * speed_mult);
        }
        if window.is_key_down(Key::Space) {
            camera.position.y += camera.speed * dt * speed_mult;
        }
        if window.is_key_down(Key::LeftCtrl) {
            camera.position.y -= camera.speed * dt * speed_mult;
        }

        // ============= MOUSE LOOK =============
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Pass) {
            if let Some((lx, ly)) = self.last_mouse_pos {
                let dx = mx - lx;
                let dy = my - ly;

                camera.yaw += dx * camera.mouse_sensitivity;
                camera.pitch -= dy * camera.mouse_sensitivity;

                // Limitar pitch
                let max_pitch = 1.5_f32;
                if camera.pitch > max_pitch {
                    camera.pitch = max_pitch;
                }
                if camera.pitch < -max_pitch {
                    camera.pitch = -max_pitch;
                }
            }
            self.last_mouse_pos = Some((mx, my));
        }
    }
}
