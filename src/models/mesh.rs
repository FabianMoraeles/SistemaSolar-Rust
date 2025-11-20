use crate::math::Vec3;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<(usize, usize, usize)>) -> Self {
        Self { vertices, faces }
    }

    pub fn scale(&mut self, factor: f32) {
        for v in &mut self.vertices {
            v.x *= factor;
            v.y *= factor;
            v.z *= factor;
        }
    }

    pub fn translate(&mut self, offset: Vec3) {
        for v in &mut self.vertices {
            v.x += offset.x;
            v.y += offset.y;
            v.z += offset.z;
        }
    }

    pub fn center(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min = self.vertices[0];
        let mut max = self.vertices[0];

        for v in &self.vertices {
            if v.x < min.x { min.x = v.x; }
            if v.y < min.y { min.y = v.y; }
            if v.z < min.z { min.z = v.z; }
            if v.x > max.x { max.x = v.x; }
            if v.y > max.y { max.y = v.y; }
            if v.z > max.z { max.z = v.z; }
        }

        let center = Vec3::new(
            (min.x + max.x) * 0.5,
            (min.y + max.y) * 0.5,
            (min.z + max.z) * 0.5,
        );

        for v in &mut self.vertices {
            v.x -= center.x;
            v.y -= center.y;
            v.z -= center.z;
        }
    }
}
