use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::ParallelHit;

pub struct FlipNormals<T> {
    pub hittable: T,
}

impl<T: ParallelHit> Hit for FlipNormals<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut rec = self.hittable.hit(ray, t_min, t_max)?;
        rec.normal = -rec.normal;
        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
