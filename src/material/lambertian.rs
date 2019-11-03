use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::prelude::ParallelTexture;

pub struct Lambertian<T> {
    pub albedo: T,
}

impl<T: ParallelTexture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = rec.p + rec.normal + crate::random_in_unit_sphere(rand::thread_rng());
        let scattered = Ray::new(rec.p, target - rec.p).with_time(r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}
