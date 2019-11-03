pub use crate::{
    vec3::Vec3,
    hit::Hit,
    texture::Texture,
    material::Material,
};

pub trait ParallelHit: Hit + Send + Sync { }
impl<T: Hit + Send + Sync> ParallelHit for T { }

pub trait ParallelTexture: Texture + Send + Sync { }
impl<T: Texture + Send + Sync> ParallelTexture for T { }

pub trait ParallelMaterial: Material + Send + Sync { }
impl<T: Material + Send + Sync> ParallelMaterial for T { }
