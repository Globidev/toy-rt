use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, Neg, Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0[0] }
    pub fn y(&self) -> f32 { self.0[1] }
    pub fn z(&self) -> f32 { self.0[2] }

    pub fn r(&self) -> f32 { self.0[0] }
    pub fn g(&self) -> f32 { self.0[1] }
    pub fn b(&self) -> f32 { self.0[2] }

    pub fn len(&self) -> f32 {
        let e = &self.0;
        (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        let e = &self.0;
        e[0] * e[0] + e[1] * e[1] + e[2] * e[2]
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x() * other.x()
        + self.y() * other.y()
        + self.z() * other.z()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        ])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        ])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
        ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3([
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        ])
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
        Vec3([
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z(),
        ])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3([
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
        ])
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
        *self = *self + rhs;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = *self / rhs
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3([
            -self.x(),
            -self.y(),
            -self.z(),
        ])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}
