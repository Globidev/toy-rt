use crate::prelude::{Hit, Material, MaterialBuilder, HitRecord, AABB, Ray, Vec3};
use crate::hit::RectBuilder;

pub struct HitBox<T> {
    pmin: Vec3,
    pmax: Vec3,
    list: HitBoxCombined<T>,
}

type HitBoxCombined<T> = impl Hit;

impl<T: Material + Clone> HitBox<T> {
    pub fn new(p0: Vec3, p1: Vec3, mat: T) -> Self {
        Self {
            pmin: p0,
            pmax: p1,
            list: Self::build_list(p0, p1, mat)
        }
    }

    fn build_list(p0: Vec3, p1: Vec3, mat: T) -> HitBoxCombined<T> {
        let (x0, y0, z0) = (p0.x(), p0.y(), p0.z());
        let (x1, y1, z1) = (p1.x(), p1.y(), p1.z());

        world![
            RectBuilder.x(x0..=x1).y(y0..=y1).z(z1).material(mat.clone()),
            RectBuilder.x(x0..=x1).y(y0..=y1).z(z0).material(mat.clone()).flip_normals(),

            RectBuilder.x(x0..=x1).z(z0..=z1).y(y1).material(mat.clone()),
            RectBuilder.x(x0..=x1).z(z0..=z1).y(y0).material(mat.clone()).flip_normals(),

            RectBuilder.y(y0..=y1).z(z0..=z1).x(x1).material(mat.clone()),
            RectBuilder.y(y0..=y1).z(z0..=z1).x(x0).material(mat.clone()).flip_normals(),
        ]
    }
}

impl<T: Material> Hit for HitBox<T> {
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
