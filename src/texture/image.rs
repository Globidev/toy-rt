use crate::prelude::{Texture, Vec3};

pub struct ImageTexture {
    pub image: image::RgbImage
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let (w, h) = self.image.dimensions();

        let mut i = (u * w as f32) as i32;
        let mut j = ((1. - v) * h as f32 - 0.001) as i32;

        if i < 0 { i = 0 }
        if j < 0 { j = 0 }
        if i > w as i32 - 1 { i = w as i32 - 1 }
        if j > h as i32 - 1 { j = h as i32 - 1 }

        let px = self.image.get_pixel(i as u32, j as u32);
        let r = px[0] as f32 / 255.;
        let g = px[1] as f32 / 255.;
        let b = px[2] as f32 / 255.;

        Vec3::new(r, g, b)
    }
}
