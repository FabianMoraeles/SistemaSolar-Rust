use super::matrix::Mat4;
use super::vector::Vec3;

/// Crea una matriz de transformación completa (TRS: Translation, Rotation, Scale)
pub fn create_transform_matrix(
    translation: Vec3,
    rotation: Vec3,  // Ángulos de Euler en radianes (pitch, yaw, roll)
    scale: Vec3,
) -> Mat4 {
    let t = Mat4::translation(translation.x, translation.y, translation.z);
    let rx = Mat4::rotation_x(rotation.x);
    let ry = Mat4::rotation_y(rotation.y);
    let rz = Mat4::rotation_z(rotation.z);
    let s = Mat4::scale(scale.x, scale.y, scale.z);

    // Orden: Escala -> Rotación (Z -> Y -> X) -> Traslación
    t * rz * ry * rx * s
}

/// Crea una matriz Model-View-Projection
pub fn create_mvp_matrix(model: Mat4, view: Mat4, projection: Mat4) -> Mat4 {
    projection * view * model
}

/// Convierte coordenadas 3D del mundo a coordenadas de pantalla
pub fn world_to_screen(
    point: Vec3,
    mvp: &Mat4,
    screen_width: f32,
    screen_height: f32,
) -> Option<(f32, f32, f32)> {
    // Transformar el punto al espacio de clip
    let clip_space = mvp.mul_point(point);
    
    // El punto está detrás de la cámara si w <= 0
    let w = mvp.mul_vec4(super::vector::Vec4::from_point(point)).w;
    if w <= 0.0 {
        return None;
    }

    // Convertir a Normalized Device Coordinates (NDC) [-1, 1]
    // Ya está dividido por w en mul_point()

    // Convertir a coordenadas de pantalla
    let screen_x = (clip_space.x + 1.0) * 0.5 * screen_width;
    let screen_y = (1.0 - clip_space.y) * 0.5 * screen_height; // Invertir Y
    let screen_z = clip_space.z; // Mantener Z para depth testing

    Some((screen_x, screen_y, screen_z))
}

/// Convierte grados a radianes
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convierte radianes a grados
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Interpola entre dos valores
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp un valor entre un mínimo y un máximo
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Interpola suavemente (smooth step)
pub fn smooth_step(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Interpola muy suavemente (smoother step)
pub fn smoother_step(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Calcula el área de un triángulo 2D (útil para rasterización)
pub fn triangle_area_2d(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
    ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)) * 0.5
}

/// Calcula coordenadas baricéntricas de un punto dentro de un triángulo
/// Retorna (u, v, w) donde u + v + w = 1
pub fn barycentric(
    p: (f32, f32),
    v0: (f32, f32),
    v1: (f32, f32),
    v2: (f32, f32),
) -> (f32, f32, f32) {
    let (px, py) = p;
    let (x0, y0) = v0;
    let (x1, y1) = v1;
    let (x2, y2) = v2;

    let denom = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2);
    
    if denom.abs() < 0.0001 {
        return (0.0, 0.0, 0.0);
    }

    let u = ((y1 - y2) * (px - x2) + (x2 - x1) * (py - y2)) / denom;
    let v = ((y2 - y0) * (px - x2) + (x0 - x2) * (py - y2)) / denom;
    let w = 1.0 - u - v;

    (u, v, w)
}

/// Verifica si un punto está dentro de un triángulo usando coordenadas baricéntricas
pub fn point_in_triangle(
    p: (f32, f32),
    v0: (f32, f32),
    v1: (f32, f32),
    v2: (f32, f32),
) -> bool {
    let (u, v, w) = barycentric(p, v0, v1, v2);
    u >= 0.0 && v >= 0.0 && w >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deg_to_rad() {
        let rad = deg_to_rad(180.0);
        assert!((rad - std::f32::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_barycentric() {
        let v0 = (0.0, 0.0);
        let v1 = (1.0, 0.0);
        let v2 = (0.0, 1.0);
        let p = (0.25, 0.25);

        let (u, v, w) = barycentric(p, v0, v1, v2);
        assert!((u + v + w - 1.0).abs() < 0.0001);
    }
}