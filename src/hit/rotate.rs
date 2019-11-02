use crate::hit::{Hit, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;

pub struct RotateY {
    pub hittable: Box<dyn Hit + Send + Sync>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl Hit for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction).with_time(ray.time);

        let mut rec = self.hittable.hit(&rotated_ray, t_min, t_max)?;

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}

impl RotateY {
    pub fn new(hittable: impl Hit + Send + Sync + 'static, angle: f32) -> Self {
        let radians = (std::f32::consts::PI / 180.) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hittable.bounding_box(0., 1.)
            .map(|bbox| compute_bbox(bbox, cos_theta, sin_theta));

        Self {
            hittable: Box::new(hittable),
            sin_theta,
            cos_theta,
            bbox
        }
    }
}

fn compute_bbox(bbox: AABB, cos_theta: f32, sin_theta: f32) -> AABB {
    let f_max = std::f32::MAX;
    let mut min = Vec3([f_max, f_max, f_max]);
    let mut max = Vec3([-f_max, -f_max, -f_max]);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;

                let x = i * bbox.max.x() + (1. - i) * bbox.min.x();
                let y = j * bbox.max.y() + (1. - j) * bbox.min.y();
                let z = k * bbox.max.z() + (1. - k) * bbox.min.z();

                let new_x = cos_theta * x + sin_theta * z;
                let new_z = -sin_theta * x + cos_theta * z;

                let tester = Vec3([new_x, y, new_z]);
                for c in 0..3 {
                    if tester[c] > max[c] {
                        max[c] = tester[c]
                    }
                    if tester[c] < min[c] {
                        min[c] = tester[c]
                    }
                }
            }
        }
    }

    AABB { min, max }
}
