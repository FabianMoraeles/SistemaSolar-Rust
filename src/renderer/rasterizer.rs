use crate::framebuffer::Framebuffer;
use crate::math::barycentric;

/// Rasteriza un triángulo lleno con z-buffer
pub fn draw_filled_triangle(
    fb: &mut Framebuffer,
    tri: [(f32, f32, f32); 3],
    color: u32,
) {
    let (x0, y0, z0) = tri[0];
    let (x1, y1, z1) = tri[1];
    let (x2, y2, z2) = tri[2];

    // Backface culling (simple, usar z del cross product)
    let normal_z = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
    if normal_z <= 0.0 {
        return;
    }

    // Bounding box
    let min_x = x0.min(x1).min(x2).floor().max(0.0) as i32;
    let max_x = x0.max(x1).max(x2).ceil().min(fb.width as f32 - 1.0) as i32;

    let min_y = y0.min(y1).min(y2).floor().max(0.0) as i32;
    let max_y = y0.max(y1).max(y2).ceil().min(fb.height as f32 - 1.0) as i32;

    for px in min_x..=max_x {
        for py in min_y..=max_y {
            let p = (px as f32, py as f32);

            let (u, v, w) = barycentric(
                p,
                (x0, y0),
                (x1, y1),
                (x2, y2),
            );

            if u < 0.0 || v < 0.0 || w < 0.0 {
                continue;
            }

            // interpolación de profundidad
            let z = u * z0 + v * z1 + w * z2;

            fb.set_pixel_with_depth(px as usize, py as usize, color, z);
        }
    }
}
