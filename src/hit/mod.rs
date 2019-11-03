use crate::{ray::Ray, vec3::Vec3};
use crate::material::Material;

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
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB>;

    fn combine<Other: Hit>(self, other: Other) -> Combine<Self, Other>
    where
        Self: Sized
    {
        Combine { a: self, b: other }
    }
}

impl<T: Hit + ?Sized> Hit for Box<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.as_ref().hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB> {
        self.as_ref().bounding_box(t0, t1)
    }
}

mod hitlist;
mod sphere;
mod moving_sphere;
mod bvh_node;
mod rect;
mod flip_normals;
mod r#box;
mod translate;
mod rotate;
mod constant_medium;
mod combine;

pub use hitlist::HitList;
pub use sphere::Sphere;
pub use moving_sphere::MovingSphere;
pub use bvh_node::BVHNode;
pub use rect::{XYRect, XZRect, YZRect};
pub use flip_normals::FlipNormals;
pub use r#box::HitBox;
pub use translate::Translate;
pub use rotate::{RotateY};
pub use constant_medium::ConstantMedium;
pub use combine::Combine;
