use crate::prelude::{Hit, AABB, HitRecord, Ray};

pub struct FlipNormals<T: Hit> {
    wrapped: T,
}

impl<T: Hit> FlipNormals<T> {
    pub fn new(wrapped: T) -> Self {
        Self { wrapped }
    }
}

impl<T: Hit> Hit for FlipNormals<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut rec = self.wrapped.hit(ray, t_min, t_max)?;
        rec.normal = -rec.normal;
        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.wrapped.bounding_box(t0, t1)
    }
}
