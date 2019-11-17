use crate::prelude::{Texture, Vec3};

pub struct Checker<OddTx: Texture, EvenTx: Texture> {
    odd: OddTx,
    even: EvenTx,
}

impl<OddTx: Texture, EvenTx: Texture> Checker<OddTx, EvenTx> {
    pub fn new(odd: OddTx, even: EvenTx) -> Self {
        Self { odd, even }
    }
}

impl<OddTx: Texture, EvenTx: Texture> Texture for Checker<OddTx, EvenTx> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines =
            (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();

        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
