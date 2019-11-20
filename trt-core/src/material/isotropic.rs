use crate::prelude::{Texture, Material, HitRecord, Ray, Vec3};
use crate::utils::random_in_unit_sphere;
pub struct Isotropic<T: Texture> {
    albedo: T
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray {
            origin: rec.p,
            direction: random_in_unit_sphere(rand::thread_rng()),
            time: r_in.time,
        };
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}
