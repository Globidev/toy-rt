use crate::prelude::{Hit, AABB, HitRecord, Ray};

pub struct HitList<T: Hit>(Vec<T>);

impl<T: Hit> HitList<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self(list)
    }
}

impl<T: Hit> Hit for HitList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        self.0.iter()
            .filter_map(|hit| {
                let rec = hit.hit(ray, t_min, closest_so_far)?;
                closest_so_far = rec.t;
                Some(rec)
            })
            .last()
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let mut hits = self.0.iter();

        let bbox = hits.next()?.bounding_box(t0, t1)?;

        hits.try_fold(bbox, |bbox_so_far, obj| {
            let bbox = obj.bounding_box(t0, t1)?;
            Some(AABB::surrounding_box(bbox_so_far, bbox))
        })
    }
}
