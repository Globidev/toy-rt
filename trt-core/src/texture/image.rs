use crate::prelude::{Texture, Vec3};

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn load(data: impl Into<Vec<u8>>, width: usize, height: usize) -> Self {
        Self {
            data: data.into(),
            width,
            height,
        }
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let x = (u * self.width as f32).max(0.) as usize;
        let y = ((1. - v) * self.height as f32 - 0.001).max(0.) as usize;

        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        let idx = y * self.width + x;
        let px = &self.data[idx * 3..];

        let r = px[0] as f32 / 255.;
        let g = px[1] as f32 / 255.;
        let b = px[2] as f32 / 255.;

        Vec3::new(r, g, b)
    }
}
