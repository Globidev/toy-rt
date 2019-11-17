#![feature(test)]
#![feature(type_alias_impl_trait)]

use rand::{random, Rng};

pub mod prelude;
pub mod hit;
pub mod material;
pub mod texture;
pub mod camera;
pub mod ray;
pub mod vec3;
pub mod aabb;
pub mod perlin;

use hit::Hit;
use ray::Ray;
use vec3::Vec3;

fn color(ray: &Ray, world: &impl Hit, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) {
                return emitted + attenuation * color(&scattered, world, depth + 1);
            }
        }

        emitted
    } else {
        Vec3::splat(0.)
    }
}

pub fn random_in_unit_sphere(mut rng: impl Rng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::random(&mut rng) - Vec3::splat(1.);
        if Vec3::dot(p, p) < 1.0 {
            break p;
        }
    }
}

pub fn random_in_unit_disk(mut rng: impl Rng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.) - Vec3::new(1., 1., 0.);
        if Vec3::dot(p, p) < 1.0 {
            break p;
        }
    }
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

pub fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    use std::f32::consts::PI;
    let phi = f32::atan2(p.z(), p.x());
    let theta = p.y().asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    (u, v)
}

fn clamp(x: f32) -> u8 {
    let clamped = if x > 255. { 255. } else { x };
    clamped as u8
}

use camera::Camera;

pub struct Color(pub u8, pub u8, pub u8);

pub fn render(world: &impl Hit, (x, y): (usize, usize), (w, h): (usize, usize), camera: &Camera, ray_count: usize) -> Color {
    let mut col = Vec3::splat(0.);
    for _r in 0..ray_count {
        let u = (x as f32 + random::<f32>()) / w as f32;
        let v = (y as f32 + random::<f32>()) / h as f32;
        let ray = camera.get_ray(u, v);
        col += color(&ray, world, 0);
    }
    col /= ray_count as f32;
    col = col.sqrt();

    let r = 255.99 * col.r();
    let g = 255.99 * col.g();
    let b = 255.99 * col.b();

    Color(clamp(r), clamp(g), clamp(b))
}
