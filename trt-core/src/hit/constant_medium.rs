use crate::prelude::{Material, Texture, Hit, AABB, HitRecord, Ray, Vec3};
use crate::material::Isotropic;

pub struct ConstantMedium<T: Hit, Mat: Material> {
    boundary: T,
    density: f32,
    phase_function: Mat,
}

impl<T: Hit, Mat: Material> ConstantMedium<T, Mat> {
    pub fn new(boundary: T, density: f32, phase_function: Mat) -> Self {
        Self { boundary, density, phase_function }
    }
}

impl<T: Hit, Tx: Texture> ConstantMedium<T, Isotropic<Tx>> {
    pub fn new_iso(boundary: T, density: f32, texture: Tx) -> Self {
        Self::new(boundary, density, Isotropic::new(texture))
    }
}

impl<T: Hit, Mat: Material> Hit for ConstantMedium<T, Mat> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let f_max = std::f32::MAX;

        let mut rec1 = self.boundary.hit(ray, -f_max, f_max)?;
        let mut rec2 = self.boundary.hit(ray, rec1.t + 0.0001, f_max)?;

        if rec1.t < t_min { rec1.t = t_min }
        if rec2.t > t_max { rec2.t = t_max }

        if rec1.t >= rec2.t {
            return None
        }

        if rec1.t < 0. { rec1.t = 0. }

        let distance_inside_boundary = (rec2.t - rec1.t) * ray.direction.len();
        let hit_distance = -(1. / self.density) * rand::random::<f32>().ln();

        if hit_distance >= distance_inside_boundary {
            return None
        }

        let t = rec1.t + hit_distance / ray.direction.len();

        Some(HitRecord {
            t,
            p: ray.point_at_parameter(t),
            normal: Vec3::new(1., 0., 0.),
            mat: &self.phase_function,
            u: 0.,
            v: 0.,
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
