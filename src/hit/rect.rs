use crate::hit::{Hit, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::ParallelMaterial;

pub struct XYRect<T> {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: T,
}

pub struct XZRect<T> {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: T,
}

pub struct YZRect<T> {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: T,
}

impl<T: ParallelMaterial> Hit for XYRect<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return None
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        }

        Some(HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            t,
            mat: &self.material,
            p: ray.point_at_parameter(t),
            normal: Vec3::new(0., 0., 1.),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vec3::new(self.x0, self.y0, self.k - 0.0001);
        let max = Vec3::new(self.x1, self.y1, self.k + 0.0001);

        Some(AABB { min, max })
    }
}

impl<T: ParallelMaterial> Hit for XZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            return None
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None
        }

        Some(HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t,
            mat: &self.material,
            p: ray.point_at_parameter(t),
            normal: Vec3::new(0., 1., 0.),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vec3::new(self.x0, self.k - 0.0001, self.z0);
        let max = Vec3::new(self.x1, self.k + 0.0001, self.z1);

        Some(AABB { min, max })
    }
}

impl<T: ParallelMaterial> Hit for YZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            return None
        }

        let y = ray.origin.y() + t * ray.direction.y();
        let z = ray.origin.z() + t * ray.direction.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None
        }

        Some(HitRecord {
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t,
            mat: &self.material,
            p: ray.point_at_parameter(t),
            normal: Vec3::new(1., 0., 0.),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vec3::new(self.k - 0.0001, self.y0, self.z0);
        let max = Vec3::new(self.k + 0.0001, self.y1, self.z1);

        Some(AABB { min, max })
    }
}
