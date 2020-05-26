use crate::prelude::{Material, AABB, Ray, Vec3};
use crate::material::Isotropic;
use crate::texture::Constant;

use std::{sync::Arc, rc::Rc};

pub struct HitRecord<'mat> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'mat dyn Material,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    fn combine<Other: Hit>(self, other: Other) -> Combine<Self, Other>
    where
        Self: Sized
    {
        Combine::new(self, other)
    }

    fn flip_normals(self) -> FlipNormals<Self>
    where
        Self: Sized
    {
        FlipNormals::new(self)
    }

    fn translate(self, offset: impl Into<Vec3>) -> Translate<Self>
    where
        Self: Sized
    {
        Translate::new(self, offset.into())
    }

    fn rotate_y(self, angle: f32) -> RotateY<Self>
    where
        Self: Sized
    {
        RotateY::new(self, angle)
    }

    fn rotate_x(self, angle: f32) -> RotateX<Self>
    where
        Self: Sized
    {
        RotateX::new(self, angle)
    }

    fn rotate_z(self, angle: f32) -> RotateZ<Self>
    where
        Self: Sized
    {
        RotateZ::new(self, angle)
    }

    fn constant_medium(self, density: f32, color: impl Into<Vec3>)
        -> ConstantMedium<Self, Isotropic<Constant>>
    where
        Self: Sized
    {
        ConstantMedium::new_iso(self, density, Constant::new(color.into()))
    }
}

impl<T: Hit + ?Sized> Hit for Box<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.as_ref().hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.as_ref().bounding_box(t0, t1)
    }
}

impl<T: Hit + ?Sized> Hit for Rc<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.as_ref().hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.as_ref().bounding_box(t0, t1)
    }
}

impl<T: Hit + ?Sized> Hit for Arc<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.as_ref().hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.as_ref().bounding_box(t0, t1)
    }
}

#[macro_export]
macro_rules! world {
    ($hit:expr) => { $hit };
    ($hit:expr, $($hits:expr),* $(,)?) => {
        $hit.combine(world!($($hits),*))
    }
}

mod combine;
pub use combine::Combine;

mod hitlist;
pub use hitlist::HitList;

mod sphere;
pub use sphere::{Sphere, SphereBuilder};

mod cylinder;
pub use cylinder::{Cylinder, CylinderBuilder};

mod moving_sphere;
pub use moving_sphere::MovingSphere;

mod bvh_node;
pub use bvh_node::BVHNode;

mod rect;
pub use rect::{Rect, RectBuilder};

mod flip_normals;
pub use flip_normals::FlipNormals;

mod r#box;
pub use r#box::HitBox;

mod translate;
pub use translate::Translate;

mod rotate;
pub use rotate::{RotateY, RotateX, RotateZ};

mod constant_medium;
pub use constant_medium::ConstantMedium;
