use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::ParallelHit;

pub struct Combine<T, U> {
    pub a: T,
    pub b: U,
}

impl<T: ParallelHit, U: ParallelHit> Hit for Combine<T, U> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.a.hit(ray, t_min, t_max) {
            Some(rec) => Some(self.b.hit(ray, t_min, rec.t).unwrap_or(rec)),
            None => self.b.hit(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox_a = self.a.bounding_box(t0, t1)?;
        let bbox_b = self.b.bounding_box(t0, t1)?;

        Some(AABB::surrounding_box(bbox_a, bbox_b))
    }
}

#[macro_export]
macro_rules! combine {
    ($hit:expr) => { $hit };
    ($hit:expr, $($hits:expr),* $(,)?) => {
        $hit.combine(combine!($($hits),*))
    }
}
