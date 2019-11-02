use crate::vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

mod constant;
mod checker;
mod noise;
mod image;

pub use constant::ConstantTexture;
pub use checker::CheckerTexture;
pub use noise::NoiseTexture;
pub use self::image::ImageTexture;
