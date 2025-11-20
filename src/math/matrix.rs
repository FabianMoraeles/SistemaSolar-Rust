use super::vector::{Vec3, Vec4};
use std::ops::Mul;

/// Matriz 4x4 para transformaciones 3D
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4 {
    /// Crea una nueva matriz con los valores dados
    pub fn new(m: [[f32; 4]; 4]) -> Self {
        Self { m }
    }

    /// Crea una matriz identidad
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de ceros
    pub fn zero() -> Self {
        Self {
            m: [[0.0; 4]; 4],
        }
    }

    /// Crea una matriz de traslación
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de escala
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            m: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de rotación alrededor del eje X
    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de rotación alrededor del eje Y
    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de rotación alrededor del eje Z
    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de rotación alrededor de un eje arbitrario
    pub fn rotation_axis(axis: Vec3, angle: f32) -> Self {
        let axis = axis.normalize();
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Self {
            m: [
                [
                    t * x * x + c,
                    t * x * y - s * z,
                    t * x * z + s * y,
                    0.0,
                ],
                [
                    t * x * y + s * z,
                    t * y * y + c,
                    t * y * z - s * x,
                    0.0,
                ],
                [
                    t * x * z - s * y,
                    t * y * z + s * x,
                    t * z * z + c,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de vista (look-at)
    /// eye: posición de la cámara
    /// center: punto al que mira la cámara
    /// up: vector up de la cámara
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = (center - eye).normalize();
        let s = f.cross(&up).normalize();
        let u = s.cross(&f);

        Self {
            m: [
                [s.x, s.y, s.z, -s.dot(&eye)],
                [u.x, u.y, u.z, -u.dot(&eye)],
                [-f.x, -f.y, -f.z, f.dot(&eye)],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Crea una matriz de proyección en perspectiva
    /// fov: campo de visión vertical en radianes
    /// aspect: relación de aspecto (width / height)
    /// near: plano cercano
    /// far: plano lejano
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_half_fov = (fov / 2.0).tan();

        Self {
            m: [
                [1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0],
                [0.0, 1.0 / tan_half_fov, 0.0, 0.0],
                [0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near)],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    /// Crea una matriz de proyección ortográfica
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            m: [
                [2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left)],
                [0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom)],
                [0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near)],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Transpone la matriz
    pub fn transpose(&self) -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[j][i];
            }
        }
        result
    }

    /// Multiplica la matriz por un Vec4
    pub fn mul_vec4(&self, v: Vec4) -> Vec4 {
        Vec4::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3] * v.w,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3] * v.w,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3] * v.w,
            self.m[3][0] * v.x + self.m[3][1] * v.y + self.m[3][2] * v.z + self.m[3][3] * v.w,
        )
    }

    /// Multiplica la matriz por un Vec3 (asume w=1 para punto)
    pub fn mul_point(&self, v: Vec3) -> Vec3 {
        let v4 = Vec4::from_point(v);
        let result = self.mul_vec4(v4);
        result.to_vec3()
    }

    /// Multiplica la matriz por un Vec3 (asume w=0 para dirección)
    pub fn mul_direction(&self, v: Vec3) -> Vec3 {
        let v4 = Vec4::from_direction(v);
        let result = self.mul_vec4(v4);
        result.to_vec3_no_divide()
    }
}

/// Multiplicación de matrices
impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = 0.0;
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let identity = Mat4::identity();
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = identity.mul_point(v);
        assert_eq!(result, v);
    }

    #[test]
    fn test_translation() {
        let translation = Mat4::translation(5.0, 10.0, 15.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = translation.mul_point(v);
        assert_eq!(result, Vec3::new(6.0, 12.0, 18.0));
    }

    #[test]
    fn test_scale() {
        let scale = Mat4::scale(2.0, 3.0, 4.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = scale.mul_point(v);
        assert_eq!(result, Vec3::new(2.0, 6.0, 12.0));
    }
}