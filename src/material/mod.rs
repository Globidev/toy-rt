use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3([0., 0., 0.])
    }
}

mod metal;
mod dielectric;
mod lambertian;
mod diffuse;
mod isotropic;

pub use metal::Metal;
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use diffuse::DiffuseLight;
pub use isotropic::Isotropic;
