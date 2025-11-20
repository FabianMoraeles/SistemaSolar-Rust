use crate::math::{Vec3, Mat4};
use crate::framebuffer::Framebuffer;

pub struct Skybox {
    stars: Vec<Vec3>,
}

impl Skybox {
    pub fn new(count: usize) -> Self {
        let mut stars = Vec::new();

        for _ in 0..count {
            // vector aleatorio normalizado
            let x = rand_range(-1.0, 1.0);
            let y = rand_range(-1.0, 1.0);
            let z = rand_range(-1.0, 1.0);
            let v = Vec3::new(x, y, z).normalize() * 300.0; // esfera grande

            stars.push(v);
        }

        Self { stars }
    }

    pub fn render(&self, fb: &mut Framebuffer, view: Mat4, projection: Mat4) {
        // Remover traslación de la vista (para que las estrellas no se muevan)
        let mut pure_view = view;
        pure_view.m[0][3] = 0.0;
        pure_view.m[1][3] = 0.0;
        pure_view.m[2][3] = 0.0;

        for star in &self.stars {
            let pos = *star;

            // MVP
            let clip = projection * pure_view * Mat4::translation(pos.x, pos.y, pos.z);

            // convertir centro de la estrella
            let p = clip.mul_point(Vec3::new(0.0, 0.0, 0.0));

            // ignorar si está detrás de la cámara
            if p.z < 0.0 {
                continue;
            }

            // convertir a pantalla
            let sx = ((p.x + 1.0) * 0.5 * fb.width as f32) as i32;
            let sy = ((1.0 - p.y) * 0.5 * fb.height as f32) as i32;

            let size = 2; // 2x2 píxeles

for dy in 0..size {
    for dx in 0..size {
        let px = sx + dx;
        let py = sy + dy;

        if px >= 0 && py >= 0 && px < fb.width as i32 && py < fb.height as i32 {
            fb.set_pixel(px as usize, py as usize, 0xFFFFFFFF);
        }
    }
}
        }
    }
}

fn rand_range(min: f32, max: f32) -> f32 {
    let r = rand::random::<f32>();
    min + r * (max - min)
}
