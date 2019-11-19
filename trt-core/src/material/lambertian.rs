use crate::prelude::{Texture, Material, HitRecord, Ray, Vec3};
use crate::texture::Constant;

pub struct Lambertian<T> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

pub struct UnboundedTx;

impl Lambertian<UnboundedTx> {
    pub fn colored(color: impl Into<Vec3>) -> Lambertian<Constant> {
        Lambertian::new(Constant::new(color.into()))
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = rec.p + rec.normal + crate::random_in_unit_sphere(rand::thread_rng());
        let scattered = Ray {
            origin: rec.p,
            direction: target - rec.p,
            time: r_in.time,
        };
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}
