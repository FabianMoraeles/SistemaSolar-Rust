use crate::math::Vec3;
use super::mesh::Mesh;

pub fn generate_sphere(lat_segments: usize, lon_segments: usize) -> Mesh {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut faces: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..=lat_segments {
        let theta = i as f32 * std::f32::consts::PI / lat_segments as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for j in 0..=lon_segments {
            let phi = j as f32 * 2.0 * std::f32::consts::PI / lon_segments as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let x = sin_theta * cos_phi;
            let y = cos_theta;
            let z = sin_theta * sin_phi;

            vertices.push(Vec3::new(x, y, z));
        }
    }

    for i in 0..lat_segments {
        for j in 0..lon_segments {
            let first = i * (lon_segments + 1) + j;
            let second = first + lon_segments + 1;

            faces.push((first, second, first + 1));
            faces.push((second, second + 1, first + 1));
        }
    }

    Mesh::new(vertices, faces)
}
