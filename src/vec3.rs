use crate::random::{random_f32, random_f32_range};
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn diagonal(value: f32) -> Self {
        Vec3::new(value, value, value)
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Self {
            e: [
                self.y() * v.z() - self.z() * v.y(),
                self.z() * v.x() - self.x() * v.z(),
                self.x() * v.y() - self.y() * v.x(),
            ],
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        let div = self.length();
        self / div
    }

    pub fn color(self, samples: f32) -> (u8, u8, u8) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();
        let scale = 1.0 / samples;

        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;
        (ir, ig, ib)
        // format!("{ir} {ig} {ib}\n")
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::diagonal(0.0)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            e: [self.x() / rhs, self.y() / rhs, self.z() / rhs],
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        (1.0 / self) * rhs
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

pub fn random() -> Vec3 {
    Vec3::new(random_f32(), random_f32(), random_f32())
}

pub fn random_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_f32_range(min..=max),
        random_f32_range(min..=max),
        random_f32_range(min..=max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let point = random_range(-1.0, 1.0);
        if point.length_squared() >= 1.0 {
            continue;
        };
        return point;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub type Point3 = Vec3;
pub type Color = Vec3;
