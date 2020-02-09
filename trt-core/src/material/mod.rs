use crate::prelude::{Ray, HitRecord, Vec3};
use std::sync::Arc;
use std::rc::Rc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::splat(0.)
    }
}

impl<T: Material + ?Sized> Material for Arc<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        self.as_ref().scatter(r_in, rec)
    }
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.as_ref().emitted(u, v, p)
    }
}

impl<T: Material + ?Sized> Material for Rc<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        self.as_ref().scatter(r_in, rec)
    }
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.as_ref().emitted(u, v, p)
    }
}

mod metal;
pub use metal::Metal;

mod dielectric;
pub use dielectric::Dielectric;

mod lambertian;
pub use lambertian::Lambertian;

mod diffuse;
pub use diffuse::Diffuse;

mod isotropic;
pub use isotropic::Isotropic;

pub mod builder;
pub use builder::{MaterialBuilder, MaterialBuilderExt};
