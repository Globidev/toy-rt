use crate::hit::{Hit, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::ParallelMaterial;

pub struct Sphere<T> {
    pub center: Vec3,
    pub radius: f32,
    pub material: T,
}

impl<T: ParallelMaterial> Hit for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                let (u, v) = crate::get_sphere_uv((p - self.center) / self.radius);
                return Some(HitRecord { t: temp, p, normal, mat: &self.material, u, v })
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                let (u, v) = crate::get_sphere_uv((p - self.center) / self.radius);
                return Some(HitRecord { t: temp, p, normal, mat: &self.material, u, v })
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let radius = self.radius;
        Some(AABB {
            min: self.center - Vec3::splat(radius),
            max: self.center + Vec3::splat(radius),
        })
    }
}
