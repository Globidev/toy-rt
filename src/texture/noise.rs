use crate::{texture::Texture, vec3::Vec3};
use crate::perlin::Perlin;

pub struct NoiseTexture {
    pub perlin: Perlin,
    pub scale: f32
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::splat(1.) * 0.5 * (1. + (self.scale * p.x() + 5. * self.perlin.turb(self.scale * p, 7)).sin())
    }
}
