use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hit::HitRecord;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = crate::reflect(r_in.direction.unit(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * crate::random_in_unit_sphere(rand::thread_rng()),
            time: 0.
        };
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction, rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
