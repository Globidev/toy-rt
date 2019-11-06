use crate::prelude::{ParallelTexture, Texture, Vec3};

pub struct Checker<OddTx, EvenTx>
where
    OddTx: ParallelTexture,
    EvenTx: ParallelTexture,
{
    odd: OddTx,
    even: EvenTx,
}

impl<OddTx, EvenTx> Checker<OddTx, EvenTx>
where
    OddTx: ParallelTexture,
    EvenTx: ParallelTexture,
{
    pub fn new(odd: OddTx, even: EvenTx) -> Self {
        Self { odd, even }
    }
}

impl<OddTx, EvenTx> Texture for Checker<OddTx, EvenTx>
where
    OddTx: ParallelTexture,
    EvenTx: ParallelTexture,
{
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
