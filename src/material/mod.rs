use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::splat(0.)
    }
}

mod metal;
pub use metal::Metal;

mod dielectric;
pub use dielectric::Dielectric;

mod lambertian;
pub use lambertian::Lambertian;

mod diffuse;
pub use diffuse::DiffuseLight;

mod isotropic;
pub use isotropic::Isotropic;
