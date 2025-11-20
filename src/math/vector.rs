use std::ops::{Add, Sub, Mul, Div, Neg};

/// Vector 3D (x, y, z)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Crea un nuevo vector 3D
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Vector cero
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Vector unitario en X
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Vector unitario en Y
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Vector unitario en Z
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Calcula la magnitud (longitud) del vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calcula la magnitud al cuadrado (más rápido, evita sqrt)
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Normaliza el vector (devuelve un vector con magnitud 1)
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            *self
        }
    }

    /// Producto punto (dot product)
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Producto cruz (cross product)
    pub fn cross(&self, other: &Vec3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Interpolación lineal entre dos vectores
    pub fn lerp(&self, other: &Vec3, t: f32) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }

    /// Distancia entre dos puntos
    pub fn distance(&self, other: &Vec3) -> f32 {
        (*self - *other).length()
    }

    /// Distancia al cuadrado (más rápido)
    pub fn distance_squared(&self, other: &Vec3) -> f32 {
        (*self - *other).length_squared()
    }

    /// Refleja el vector respecto a una normal
    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - *normal * (2.0 * self.dot(normal))
    }
}

// Implementación de operadores
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(vec.x * self, vec.y * self, vec.z * self)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

/// Vector 4D (x, y, z, w) - usado para transformaciones homogéneas
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Crea un nuevo vector 4D
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Crea un Vec4 desde un Vec3 con w=1 (punto)
    pub fn from_point(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z, 1.0)
    }

    /// Crea un Vec4 desde un Vec3 con w=0 (dirección)
    pub fn from_direction(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z, 0.0)
    }

    /// Convierte a Vec3 dividiendo por w (perspectiva)
    pub fn to_vec3(&self) -> Vec3 {
        if self.w != 0.0 {
            Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
        } else {
            Vec3::new(self.x, self.y, self.z)
        }
    }

    /// Convierte a Vec3 sin dividir por w
    pub fn to_vec3_no_divide(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let sum = v1 + v2;
        assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));

        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < 0.0001);
    }
}