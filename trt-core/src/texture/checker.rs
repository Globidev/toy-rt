use crate::prelude::{Texture, Vec3};

pub struct Checker<OddTx: Texture, EvenTx: Texture> {
    odd: OddTx,
    even: EvenTx,
    repeat_frequency: f32,
}

impl<OddTx: Texture, EvenTx: Texture> Checker<OddTx, EvenTx> {
    pub fn new(odd: OddTx, even: EvenTx, repeat_frequency: f32) -> Self {
        Self { odd, even, repeat_frequency }
    }
}

impl<OddTx: Texture, EvenTx: Texture> Texture for Checker<OddTx, EvenTx> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let p = self.repeat_frequency * p;
        let sines = p.x().sin() * p.y().sin() * p.z().sin();

        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
