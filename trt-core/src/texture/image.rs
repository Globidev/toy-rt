use crate::prelude::{Texture, Vec3};
use std::path::Path;

pub struct Image {
    image: image::RgbImage
}

impl Image {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ImageLoadError> {
        let img = image::open(path)
            .map_err(ImageLoadError::OpenError)?;

        Ok(Self { image: img.into_rgb() })
    }

    pub fn load_from_memory(data: &[u8]) -> Result<Self, ImageLoadError> {
        let img = image::load_from_memory(data)
            .map_err(ImageLoadError::OpenError)?;

        Ok(Self { image: img.into_rgb() })
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let (w, h) = self.image.dimensions();

        let i = (u * w as f32) as i32;
        let j = ((1. - v) * h as f32 - 0.001) as i32;

        let i = (i.max(0) as u32).min(w - 1);
        let j = (j.max(0) as u32).min(h - 1);

        let px = self.image.get_pixel(i, j);

        let r = px[0] as f32 / 255.;
        let g = px[1] as f32 / 255.;
        let b = px[2] as f32 / 255.;

        Vec3::new(r, g, b)
    }
}

#[derive(Debug)]
pub enum ImageLoadError {
    OpenError(image::ImageError),
}
