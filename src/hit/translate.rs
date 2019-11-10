use crate::hit::HitRecord;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::Hit;

pub struct Translate<T: Hit> {
    wrapped: T,
    offset: Vec3,
}

impl<T: Hit> Translate<T> {
    pub fn new(wrapped: T, offset: Vec3) -> Self {
        Self { wrapped, offset }
    }
}

impl<T: Hit> Hit for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let moved_ray = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };
        let mut rec = self.wrapped.hit(&moved_ray, t_min, t_max)?;
        rec.p += self.offset;
        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = self.wrapped.bounding_box(t0, t1)?;
        Some(AABB {
            min: bbox.min + self.offset,
            max: bbox.max + self.offset,
        })
    }
}
