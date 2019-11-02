use crate::hit::{Hit, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;

pub struct Translate {
    pub hittable: Box<dyn Hit + Send + Sync>,
    pub offset: Vec3,
}

impl Hit for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction).with_time(ray.time);
        let mut rec = self.hittable.hit(&moved_ray, t_min, t_max)?;
        rec.p += self.offset;
        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = self.hittable.bounding_box(t0, t1)?;
        Some(AABB {
            min: bbox.min + self.offset,
            max: bbox.max + self.offset,
        })
    }
}
