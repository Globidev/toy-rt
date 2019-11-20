pub use crate::{
    aabb::AABB,
    color::Color,
    dimension::{Dimension, X, Y, Z},
    hit::{Hit, HitRecord},
    material::{Material, MaterialBuilder, MaterialBuilderExt},
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};

pub trait ParallelHit: Hit + Send + Sync {}
impl<T: Hit + Send + Sync> ParallelHit for T {}

pub trait Asf32: num_traits::AsPrimitive<f32> {}
impl<T: num_traits::AsPrimitive<f32>> Asf32 for T {}
