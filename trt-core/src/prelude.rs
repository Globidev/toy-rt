pub use crate::{
    vec3::Vec3,
    ray::Ray,
    aabb::AABB,
    hit::{Hit, HitRecord},
    texture::Texture,
    material::{Material, MaterialBuilder, MaterialBuilderExt},
    dimension::{Dimension, X, Y, Z},
};

pub trait ParallelHit: Hit + Send + Sync { }
impl<T: Hit + Send + Sync> ParallelHit for T { }

pub trait Asf32: num_traits::AsPrimitive<f32> { }
impl<T: num_traits::AsPrimitive<f32>> Asf32 for T { }
