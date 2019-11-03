use crate::hit::{Hit, HitRecord, HitList, XYRect, XZRect, YZRect, FlipNormals};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::prelude::{ParallelHit, ParallelMaterial};

pub struct HitBox {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub list: HitList<Box<dyn ParallelHit>>,
}

impl HitBox {
    pub fn new(p0: Vec3, p1: Vec3, mat: impl ParallelMaterial + Clone + 'static) -> Self {
        let list = HitList::new_dyn(vec![
            Box::new(XYRect { x0: p0.x(), x1: p1.x(), y0: p0.y(), y1: p1.y(), k: p1.z(), material: mat.clone() }),
            Box::new(FlipNormals {
                hittable: XYRect { x0: p0.x(), x1: p1.x(), y0: p0.y(), y1: p1.y(), k: p0.z(), material: mat.clone() }
            }),
            Box::new(XZRect { x0: p0.x(), x1: p1.x(), z0: p0.z(), z1: p1.z(), k: p1.y(), material: mat.clone() }),
            Box::new(FlipNormals {
                hittable: XZRect { x0: p0.x(), x1: p1.x(), z0: p0.z(), z1: p1.z(), k: p0.y(), material: mat.clone() }
            }),
            Box::new(YZRect { y0: p0.y(), y1: p1.y(), z0: p0.z(), z1: p1.z(), k: p1.x(), material: mat.clone() }),
            Box::new(FlipNormals {
                hittable: YZRect { y0: p0.y(), y1: p1.y(), z0: p0.z(), z1: p1.z(), k: p0.x(), material: mat }
            }),
        ]);

        Self {
            pmin: p0,
            pmax: p1,
            list
        }
    }
}

impl Hit for HitBox {
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
