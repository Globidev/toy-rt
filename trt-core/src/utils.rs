use crate::prelude::{Hit, Ray, Vec3};

use rand::Rng;

pub fn compute_color(ray: &Ray, world: &impl Hit, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) {
                return emitted + attenuation * compute_color(&scattered, world, depth + 1);
            }
        }

        emitted
    } else {
        Vec3::splat(0.)
    }
}

pub fn random_in_unit_sphere(mut rng: impl Rng) -> Vec3 {
    std::iter::repeat_with(|| 2.0 * Vec3::random(&mut rng) - Vec3::splat(1.))
        .find(|p| Vec3::dot(*p, *p) < 1.0)
        .unwrap()
}

pub fn random_in_unit_disk(mut rng: impl Rng) -> Vec3 {
    std::iter::repeat_with(|| 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.) - Vec3::new(1., 1., 0.))
        .find(|p| Vec3::dot(*p, *p) < 1.0)
        .unwrap()
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
