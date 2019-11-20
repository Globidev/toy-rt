use crate::prelude::Vec3;

pub struct Color(pub u8, pub u8, pub u8);

impl From<Vec3> for Color {
    fn from(vec: Vec3) -> Self {
        let as_rgb = (vec * 255.99).min(Vec3::splat(255));
        let (r, g, b) = (as_rgb.x(), as_rgb.y(), as_rgb.z());

        Self(r as u8, g as u8, b as u8)
    }
}
