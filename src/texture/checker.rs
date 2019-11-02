use crate::{texture::Texture, vec3::Vec3};

pub struct CheckerTexture {
    pub odd: Box<dyn Texture + Send + Sync>,
    pub even: Box<dyn Texture + Send + Sync>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();

        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
