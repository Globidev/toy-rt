use crate::prelude::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

mod constant;
pub use constant::Constant;

mod checker;
pub use checker::Checker;

mod noise;
pub use noise::Noise;

mod image;
pub use self::image::{Image, ImageLoadError};

