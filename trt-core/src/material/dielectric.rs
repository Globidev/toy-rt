use crate::prelude::{Material, HitRecord, Ray, Vec3};

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = crate::reflect(r_in.direction, rec.normal);
        let attenuation = Vec3::splat(1.);

        let (outward_normal, ni_over_nt, cosine) =
            if Vec3::dot(r_in.direction, rec.normal) > 0. {
                let cosine = self.ref_idx * Vec3::dot(r_in.direction, rec.normal) / r_in.direction.len();
                (-rec.normal, self.ref_idx, cosine)
            } else {
                let cosine = -Vec3::dot(r_in.direction, rec.normal) / r_in.direction.len();
                (rec.normal, 1.0 / self.ref_idx, cosine)
            };

        let prob = rand::random::<f32>();

        if let Some(refracted) = crate::refract(r_in.direction, outward_normal, ni_over_nt) {
            if prob >= crate::schlick(cosine, self.ref_idx) {
                let scattered = Ray {
                    origin: rec.p,
                    direction: refracted,
                    time: 0.,
                };
                return Some((scattered, attenuation))
            }
        }

        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
            time: 0.,
        };
        Some((scattered, attenuation))
    }
}
