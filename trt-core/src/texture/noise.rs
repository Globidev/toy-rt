use crate::prelude::{Texture, Vec3};
use crate::perlin::Perlin;

pub struct Noise {
    perlin: Perlin,
    scale: f32
}

impl Noise {
    pub fn from_scale(scale: f32) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::splat(1.) * 0.5 * (1. + (self.scale * p.x() + 5. * self.perlin.turb(self.scale * p, 7)).sin())
    }
}
