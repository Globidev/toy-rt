use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let min = (self.min - ray.origin) / ray.direction;
        let max = (self.max - ray.origin) / ray.direction;

        let mins = min.min(max);
        let maxs = min.max(max);

        let tmin = mins.max_element(tmin);
        let tmax = maxs.min_element(tmax);

        tmax > tmin
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vec3::min(box0.min, box1.min);
        let big = Vec3::max(box0.max, box1.max);

        AABB {
            min: small,
            max: big
        }
    }
}
