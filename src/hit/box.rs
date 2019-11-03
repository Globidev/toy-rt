use crate::hit::{Hit, HitRecord, XYRect, XZRect, YZRect, FlipNormals};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::{ParallelHit, ParallelMaterial};

pub struct HitBox<T> {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub list: HitBoxCombined<T>,
}

type HitBoxCombined<T> = impl ParallelHit;

impl<T: ParallelMaterial + Clone> HitBox<T> {
    pub fn new(p0: Vec3, p1: Vec3, mat: T) -> Self {
        Self {
            pmin: p0,
            pmax: p1,
            list: Self::build_list(p0, p1, mat)
        }
    }

    fn build_list(p0: Vec3, p1: Vec3, mat: T) -> HitBoxCombined<T> {
        combine!(
            XYRect { x0: p0.x(), x1: p1.x(), y0: p0.y(), y1: p1.y(), k: p1.z(), material: mat.clone() },
            FlipNormals {
                hittable: XYRect { x0: p0.x(), x1: p1.x(), y0: p0.y(), y1: p1.y(), k: p0.z(), material: mat.clone() }
            },
            XZRect { x0: p0.x(), x1: p1.x(), z0: p0.z(), z1: p1.z(), k: p1.y(), material: mat.clone() },
            FlipNormals {
                hittable: XZRect { x0: p0.x(), x1: p1.x(), z0: p0.z(), z1: p1.z(), k: p0.y(), material: mat.clone() }
            },
            YZRect { y0: p0.y(), y1: p1.y(), z0: p0.z(), z1: p1.z(), k: p1.x(), material: mat.clone() },
            FlipNormals {
                hittable: YZRect { y0: p0.y(), y1: p1.y(), z0: p0.z(), z1: p1.z(), k: p0.x(), material: mat }
            },
        )
    }
}

impl<T: ParallelMaterial> Hit for HitBox<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.list.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.pmin,
            max: self.pmax,
        })
    }
}
