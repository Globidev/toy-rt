pub use crate::{
    vec3::Vec3,
    ray::Ray,
    aabb::AABB,
    hit::{Hit, HitRecord},
    texture::Texture,
    material::Material,
};

pub trait ParallelHit: Hit + Send + Sync { }
impl<T: Hit + Send + Sync> ParallelHit for T { }
