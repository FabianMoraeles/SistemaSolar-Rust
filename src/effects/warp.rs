use crate::math::{Vec3, smooth_step};

pub struct WarpEffect {
    pub active: bool,
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub time: f32,
    pub duration: f32,
}

impl WarpEffect {
    pub fn new() -> Self {
        Self {
            active: false,
            start_pos: Vec3::zero(),
            end_pos: Vec3::zero(),
            time: 0.0,
            duration: 1.5,
        }
    }

    pub fn start(&mut self, from: Vec3, to: Vec3) {
        self.active = true;
        self.start_pos = from;
        self.end_pos = to;
        self.time = 0.0;
    }

    pub fn update(&mut self, dt: f32, camera_pos: &mut Vec3) {
        if !self.active {
            return;
        }

        self.time += dt;

        let t = (self.time / self.duration).clamp(0.0, 1.0);
        let smooth = smooth_step(t);

        // interpolate camera
        *camera_pos = self.start_pos.lerp(&self.end_pos, smooth);

        if t >= 1.0 {
            self.active = false;
        }
    }
}
