#[cfg(not(target_arch = "wasm32"))]
pub use simd::Vec3;
#[cfg(target_arch = "wasm32")]
pub use array::Vec3;

pub mod simd;
pub mod array;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Vec3;
    use test::Bencher;

    use rand::{rngs::StdRng, SeedableRng};

    #[bench]
    fn dot(bencher: &mut Bencher) {
        let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF);
        let v1 = Vec3::random(&mut rng);

        let vecs = std::iter::repeat_with(|| Vec3::random(&mut rng))
            .take(10_000)
            .collect::<Vec<_>>();

        bencher.iter(move || vecs.iter().fold(0., |s, &v2| s + v1.dot(v2)))
    }

    #[bench]
    fn cross(bencher: &mut Bencher) {
        let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF);
        let v1 = Vec3::random(&mut rng);

        let vecs = std::iter::repeat_with(|| Vec3::random(&mut rng))
            .take(10_000)
            .collect::<Vec<_>>();

        bencher.iter(move || {
            vecs.iter()
                .fold(0., |s, &v2| s + v1.cross(v2).squared_len())
        })
    }
}
