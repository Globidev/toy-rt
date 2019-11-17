use crate::prelude::{Texture, Vec3};

pub struct Constant {
    color: Vec3,
}

impl Constant {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for Constant {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color
    }
}
