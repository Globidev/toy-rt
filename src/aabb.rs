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
                (self.min[a] - ray.origin[a]) / ray.direction[a],
                (self.max[a] - ray.origin[a]) / ray.direction[a],
            );

            let t1 = ffmax(
                (self.min[a] - ray.origin[a]) / ray.direction[a],
                (self.max[a] - ray.origin[a]) / ray.direction[a],
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
        let small = Vec3([
            ffmin(box0.min.x(), box1.min.x()),
            ffmin(box0.min.y(), box1.min.y()),
            ffmin(box0.min.z(), box1.min.z()),
        ]);

        let big = Vec3([
            ffmax(box0.max.x(), box1.max.x()),
            ffmax(box0.max.y(), box1.max.y()),
            ffmax(box0.max.z(), box1.max.z()),
        ]);

        AABB {
            min: small,
            max: big
        }
    }
}
