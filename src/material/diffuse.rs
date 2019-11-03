use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::prelude::ParallelTexture;

pub struct DiffuseLight<T> {
    pub emit: T,
}

impl<T: ParallelTexture> Material for DiffuseLight<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        // let target = rec.p + rec.normal + crate::random_in_unit_sphere();
        // let scattered = Ray::new(rec.p, target - rec.p).with_time(r_in.time);
        // let attenuation = self.albedo.value(0., 0., rec.p);
        // Some((scattered, attenuation))
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
