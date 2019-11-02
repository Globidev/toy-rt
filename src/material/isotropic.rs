use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::texture::Texture;

pub struct Isotropic {
    pub albedo: Box<dyn Texture + Send + Sync>,
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(rec.p, crate::random_in_unit_sphere()).with_time(r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((scattered, attenuation))
    }
}
