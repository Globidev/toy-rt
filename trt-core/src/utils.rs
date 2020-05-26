use crate::prelude::{Hit, Ray, Vec3};

pub use rand::{Rng, thread_rng, seq::SliceRandom, distributions::Distribution};

pub fn compute_color(mut ray: Ray, world: &impl Hit, ambiant_color: Vec3, max_depth: usize) -> Vec3 {
    let mut components = Vec::with_capacity(max_depth);

    for _depth in 0..max_depth {
        if let Some(rec) = world.hit(&ray, 0.001, std::f32::MAX) {
            let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);

            if let Some((scattered, attenuation)) = rec.mat.scatter(&ray, &rec) {
                components.push((emitted, attenuation));
                ray = scattered;
            } else {
                return components.into_iter().rev()
                    .fold(emitted, |r, (e, a)| r * a + e)
            }
        } else {
            break
        }
    }

    components.into_iter().rev()
        .fold(ambiant_color, |r, (e, a)| r * a + e)
}

pub fn random_in_unit_sphere(mut rng: impl Rng) -> Vec3 {
    let [x, y, z]: [f32; 3] = rand_distr::UnitSphere.sample(&mut rng);
    Vec3::new(x, y, z)
}

pub fn random_in_unit_disk(mut rng: impl Rng) -> Vec3 {
    let [x, y]: [f32; 2] = rand_distr::UnitDisc.sample(&mut rng);
    Vec3::new(x, y, 0)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;

    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub fn sphere_uv(p: Vec3) -> (f32, f32) {
    use std::f32::consts::PI;
    let phi = f32::atan2(p.z(), p.x());
    let theta = p.y().asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    (u, v)
}

pub fn cylinder_uv(p: Vec3) -> (f32, f32) {
    use std::f32::consts::PI;
    let phi = f32::atan2(p.z(), p.x());
    let u = 1. - (phi + PI) / (2. * PI);
    let v = p.y() % 1.;
    (u, v)
}
