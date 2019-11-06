use rand::{Rng, thread_rng};
use crate::vec3::Vec3;

pub struct Perlin {
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
    ranvec: [Vec3; 256],
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            ranvec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;

        let mut c = [[[Vec3::splat(0.); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ]
                }
            }
        }

        trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Vec3, depth: usize) -> f32 {
        let mut accum = 0.;
        let mut temp_p = p;
        let mut weight = 1.;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.;
        }
        accum.abs()
    }
}

fn perlin_generate() -> [Vec3; 256] {
    let mut p = [Vec3::splat(0.); 256];
    let mut rng = thread_rng();
    for v in &mut p[..] {
        let x = 2. * rng.gen::<f32>() - 1.;
        let y = 2. * rng.gen::<f32>() - 1.;
        let z = 2. * rng.gen::<f32>() - 1.;
        *v = Vec3::new(x, y, z).unit();
    }
    p
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut p = [0; 256];
    for (i, x) in p.iter_mut().enumerate() {
        *x = i
    }
    permute(&mut p[..]);
    p
}

fn permute(slice: &mut [usize]) {
    let mut rng = thread_rng();

    for i in (1..slice.len()).rev() {
        let target = rng.gen_range(0, i + 1);
        slice.swap(i, target)
    }
}

fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1 - i) as f32 * (1. - uu)) *
                         (j as f32 * vv + (1 - j) as f32 * (1. - vv)) *
                         (k as f32 * ww + (1 - k) as f32 * (1. - ww)) * Vec3::dot(c[i][j][k], weight_v);
            }
        }
    }
    accum
}
