use crate::prelude::{Hit, AABB, HitRecord, Ray, Vec3};

pub struct RotateY<T: Hit> {
    hittable: T,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl<T: Hit> RotateY<T> {
    pub fn new(hittable: T, angle: f32) -> Self {
        let radians = (std::f32::consts::PI / 180.) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hittable.bounding_box(0., 1.)
            .map(|bbox| compute_bbox(bbox, cos_theta, sin_theta));

        Self {
            hittable,
            sin_theta,
            cos_theta,
            bbox
        }
    }
}

impl<T: Hit> Hit for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin.set(0, self.cos_theta * ray.origin.x() - self.sin_theta * ray.origin.z());
        origin.set(2, self.sin_theta * ray.origin.x() + self.cos_theta * ray.origin.z());

        direction.set(0, self.cos_theta * ray.direction.x() - self.sin_theta * ray.direction.z());
        direction.set(2, self.sin_theta * ray.direction.x() + self.cos_theta * ray.direction.z());

        let rotated_ray = Ray {
            origin,
            direction,
            time: ray.time,
        };

        let mut rec = self.hittable.hit(&rotated_ray, t_min, t_max)?;

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.set(0, self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z());
        p.set(2, -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z());

        normal.set(0, self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z());
        normal.set(2, -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z());

        rec.p = p;
        rec.normal = normal;

        Some(rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}

fn compute_bbox(bbox: AABB, cos_theta: f32, sin_theta: f32) -> AABB {
    let f_max = std::f32::MAX;

    let mut min = Vec3::splat(f_max);
    let mut max = Vec3::splat(-f_max);

    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;

                let x = i * bbox.max.x() + (1. - i) * bbox.min.x();
                let y = j * bbox.max.y() + (1. - j) * bbox.min.y();
                let z = k * bbox.max.z() + (1. - k) * bbox.min.z();

                let new_x = cos_theta * x + sin_theta * z;
                let new_z = -sin_theta * x + cos_theta * z;

                let tester = Vec3::new(new_x, y, new_z);
                max = Vec3::max(tester, max);
                min = Vec3::min(tester, min);
            }
        }
    }

    AABB { min, max }
}
