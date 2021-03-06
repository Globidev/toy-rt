use crate::prelude::{Asf32, Dimension, X, Y, Z};
use crate::utils::Rng;
use packed_simd::{f32x4, shuffle};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(f32x4);

impl Vec3 {
    pub fn new(x: impl Asf32, y: impl Asf32, z: impl Asf32) -> Self {
        Self(f32x4::new(x.as_(), y.as_(), z.as_(), 0.))
    }

    pub fn splat(xyz: impl Asf32) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub fn random(mut rng: impl Rng) -> Self {
        Self(f32x4::from_slice_aligned(&rng.gen::<[f32; 4]>()))
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

    pub fn set<D: Dimension>(self, value: f32) -> Self {
        Self(unsafe { self.0.replace_unchecked(D::INDEX, value) })
    }

    pub fn get<D: Dimension>(&self) -> f32 {
        unsafe { self.0.extract_unchecked(D::INDEX) }
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        (self.0 * self.0).sum()
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn sqrt(self) -> Vec3 {
        Self(self.0.sqrt())
    }

    pub fn dot(self, other: Vec3) -> f32 {
        (self.0 * other.0).sum()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        let Self(a) = self;
        let Self(b) = other;

        let r1: f32x4 = shuffle!(a, [1, 2, 0, 3]);
        let r2: f32x4 = shuffle!(b, [2, 0, 1, 3]);
        let r3: f32x4 = shuffle!(a, [2, 0, 1, 3]);
        let r4: f32x4 = shuffle!(b, [1, 2, 0, 3]);

        Self((r1 * r2) - (r3 * r4))
    }

    pub fn min(self, other: Self) -> Vec3 {
        Self(self.0.min(other.0))
    }

    pub fn max(self, other: Self) -> Vec3 {
        Self(self.0.max(other.0))
    }

    pub fn min_element(self, fourth: f32) -> f32 {
        unsafe { self.0.replace_unchecked(3, fourth).min_element() }
    }

    pub fn max_element(self, fourth: f32) -> f32 {
        unsafe { self.0.replace_unchecked(3, fourth).max_element() }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs)
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
        Vec3(self.0 / rhs.0)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs)
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
        self.0 += rhs.0
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0)
    }
}

impl<A: Asf32, B: Asf32, C: Asf32> From<(A, B, C)> for Vec3 {
    fn from((x, y, z): (A, B, C)) -> Self {
        Self::new(x, y, z)
    }
}
