use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

use crate::{ffmin, ffmax};

impl AABB {
    pub fn hit(&self, ray: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for a in 0..3 {
            let t0 = ffmin(
                (self.min.get(a) - ray.origin.get(a)) / ray.direction.get(a),
                (self.max.get(a) - ray.origin.get(a)) / ray.direction.get(a),
            );

            let t1 = ffmax(
                (self.min.get(a) - ray.origin.get(a)) / ray.direction.get(a),
                (self.max.get(a) - ray.origin.get(a)) / ray.direction.get(a),
            );

            tmin = ffmax(t0, tmin);
            tmax = ffmin(t1, tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
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
