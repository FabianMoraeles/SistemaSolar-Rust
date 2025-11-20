use crate::math::{Vec3, Vec4, Mat4};
use crate::renderer::rasterizer::draw_filled_triangle;
use crate::framebuffer::Framebuffer;

pub struct Pipeline {
    pub mvp: Mat4,
    pub color: u32,
    pub fb_width: f32,
    pub fb_height: f32,
}

impl Pipeline {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            mvp: Mat4::identity(),
            color: 0xFFFFFFFF,
            fb_width: width as f32,
            fb_height: height as f32,
        }
    }

    pub fn set_mvp(&mut self, mvp: Mat4) {
        self.mvp = mvp;
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    /// Dibuja una malla completa (triángulos)
    pub fn draw_mesh(
        &self,
        fb: &mut Framebuffer,
        vertices: &Vec<Vec3>,
        faces: &Vec<(usize, usize, usize)>
    ) {
        for (i0, i1, i2) in faces {
            let v0 = vertices[*i0];
            let v1 = vertices[*i1];
            let v2 = vertices[*i2];

            let p0 = self.project(v0);
            let p1 = self.project(v1);
            let p2 = self.project(v2);

            if let (Some(a), Some(b), Some(c)) = (p0, p1, p2) {
                draw_filled_triangle(fb, [a, b, c], self.color);
            }
        }
    }

    #[inline]
    fn project(&self, p: Vec3) -> Option<(f32,f32,f32)> {
        let clip = self.mvp.mul_vec4(Vec4::from_point(p));

        if clip.w <= 0.0 {
            return None;
        }

        let ndc_x = clip.x / clip.w;
        let ndc_y = clip.y / clip.w;
        let ndc_z = clip.z / clip.w;

        let sx = (ndc_x + 1.0) * 0.5 * self.fb_width;
        let sy = (1.0 - ndc_y) * 0.5 * self.fb_height;

        Some((sx, sy, ndc_z))
    }
}
