use crate::prelude::{Asf32, Dimension, X, Y, Z};
use crate::utils::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3([f32; 4]);

impl Vec3 {
    pub fn new(x: impl Asf32, y: impl Asf32, z: impl Asf32) -> Self {
        Self([x.as_(), y.as_(), z.as_(), 0.])
    }

    pub fn splat(xyz: impl Asf32) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub fn random(mut rng: impl Rng) -> Self {
        Self(rng.gen())
    }

    pub fn x(&self) -> f32 {
        self.get::<X>()
    }
    pub fn y(&self) -> f32 {
        self.get::<Y>()
    }
    pub fn z(&self) -> f32 {
        self.get::<Z>()
    }

    pub fn set<D: Dimension>(mut self, value: f32) -> Self {
        self.0[D::INDEX] = value;
        self
    }

    pub fn get<D: Dimension>(&self) -> f32 {
        self.0[D::INDEX]
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn sqrt(self) -> Vec3 {
        Self([self.x().sqrt(), self.y().sqrt(), self.z().sqrt(), 0.])
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Self([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
            0.,
        ])
    }

    pub fn min(self, other: Self) -> Vec3 {
        Self([
            self.x().min(other.x()),
            self.y().min(other.y()),
            self.z().min(other.z()),
            0.,
        ])
    }

    pub fn max(self, other: Self) -> Vec3 {
        Self([
            self.x().max(other.x()),
            self.y().max(other.y()),
            self.z().max(other.z()),
            0.,
        ])
    }

    pub fn min_element(self, _fourth: f32) -> f32 {
        self.x().min(self.y()).min(self.z())
    }

    pub fn max_element(self, _fourth: f32) -> f32 {
        self.x().max(self.y()).max(self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self([
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            0.,
        ])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self([
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            0.,
        ])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self([
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
            0.,
        ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self([self.x() * rhs, self.y() * rhs, self.z() * rhs, 0.])
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self([
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z(),
            0.,
        ])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self([self.x() / rhs, self.y() / rhs, self.z() / rhs, 0.])
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0[0] /= rhs.0[0];
        self.0[1] /= rhs.0[1];
        self.0[2] /= rhs.0[2];
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self([-self.x(), -self.y(), -self.z(), 0.])
    }
}

impl<A: Asf32, B: Asf32, C: Asf32> From<(A, B, C)> for Vec3 {
    fn from((x, y, z): (A, B, C)) -> Self {
        Self::new(x, y, z)
    }
}
