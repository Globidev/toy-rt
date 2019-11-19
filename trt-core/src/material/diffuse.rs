use crate::prelude::{Texture, Material, HitRecord, Ray, Vec3};
use crate::texture::Constant;

pub struct Diffuse<T> {
    emit: T,
}

impl<T: Texture> Diffuse<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

pub struct UnboundedTx;

impl Diffuse<UnboundedTx> {
    pub fn colored(color: impl Into<Vec3>) -> Diffuse<Constant> {
        Diffuse::new(Constant::new(color.into()))
    }
}

impl<T: Texture> Material for Diffuse<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
