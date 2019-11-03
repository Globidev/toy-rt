use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::ParallelHit;

pub struct HitList<T>(Vec<T>);

impl HitList<Box<dyn ParallelHit>> {
    pub fn new_dyn(list: Vec<Box<dyn ParallelHit>>) -> Self {
        Self(list)
    }
}

impl<T: ParallelHit> Hit for HitList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        for hit in &self.0 {
            if let Some(rec) = hit.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                record = Some(rec);
            }
        }

        record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.0.is_empty() {
            return None
        }

        let mut bbox = self.0[0].bounding_box(t0, t1)?;

        for obj in self.0.iter().skip(1) {
            let temp_box = obj.bounding_box(t0, t1)?;
            bbox = AABB::surrounding_box(bbox, temp_box);
        }

        Some(bbox)
    }
}
