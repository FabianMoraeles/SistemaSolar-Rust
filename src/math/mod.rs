// Módulo de matemáticas para el renderer 3D

pub mod vector;
pub mod matrix;
pub mod transforms;

// Re-exportar los tipos más usados
pub use vector::{Vec3, Vec4};
pub use matrix::Mat4;
pub use transforms::*;