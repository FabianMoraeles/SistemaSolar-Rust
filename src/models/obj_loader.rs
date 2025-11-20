use crate::math::Vec3;
use super::mesh::Mesh;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_obj(path: &str, scale: f32) -> Mesh {
    let file = File::open(path).expect("No se pudo abrir el archivo OBJ");
    let reader = BufReader::new(file);

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut faces: Vec<(usize, usize, usize)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("v ") {
            let p: Vec<&str> = line.split_whitespace().collect();
            let x = p[1].parse::<f32>().unwrap();
            let y = p[2].parse::<f32>().unwrap();
            let z = p[3].parse::<f32>().unwrap();
            vertices.push(Vec3::new(x, y, z));
        }

        if line.starts_with("f ") {
            let p: Vec<&str> = line.split_whitespace().collect();

            let a = p[1].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
            let b = p[2].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
            let c = p[3].split('/').next().unwrap().parse::<usize>().unwrap() - 1;

            faces.push((a, b, c));
        }
    }

    let mut mesh = Mesh::new(vertices, faces);
    mesh.center();
    mesh.scale(scale);

    mesh
}
