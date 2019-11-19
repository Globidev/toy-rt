use crate::prelude::Vec3;
use crate::material::{Metal, Dielectric, DiffuseLight};
use crate::texture::Constant;

pub trait MaterialBuilder<Mat>: Sized {
    type Finished;

    fn material(self, material: Mat) -> Self::Finished;
}

pub trait MaterialBuilderExt {
    fn dielectric(self, ref_idx: f32) -> Self::Finished
    where
        Self: MaterialBuilder<Dielectric>,
    {
        self.material(Dielectric::new(ref_idx))
    }

    fn diffuse_color(self, color: impl Into<Vec3>) -> Self::Finished
    where
        Self: MaterialBuilder<DiffuseLight<Constant>>,
    {
        self.material(DiffuseLight::new(Constant::new(color.into())))
    }

    fn metallic(self, albedo: impl Into<Vec3>) -> Self::Finished
    where
        Self: MaterialBuilder<Metal>,
    {
        self.material(Metal::new(albedo.into(), 0_f32))
    }
}

impl<T> MaterialBuilderExt for T { }
