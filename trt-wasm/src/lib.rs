use wasm_bindgen::prelude::*;

use trt_core::prelude::*;
use trt_core::scene::Scene as SceneImpl;
use trt_core::camera::CameraBuilder;
use trt_core::hit::{Sphere, MovingSphere, RectBuilder, HitBox, BVHNode};
use trt_core::material::Lambertian;
use trt_core::texture::Noise;
use trt_core::world;
use rand::random;

use std::sync::Arc;

#[wasm_bindgen]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once()
}

#[wasm_bindgen]
pub struct Scene(SceneImpl<Box<dyn Hit>>);

#[wasm_bindgen]
impl Scene {
    pub fn new(source: &str) -> Self {
        const WIDTH: usize = 300;
        const HEIGHT: usize = 300;
        const RAYS_PER_PX: usize = 500;

        let camera = CameraBuilder::default()
            // .look_from((478, 278, -600))
            // .look_at((278, 278, 0))

            // .look_from((278, 278, -800))
            // .look_at((278, 278, 0))

            .look_at((0, 0, 0))
            .look_from((0, 0, -200))

            .dimensions(WIDTH as f32, HEIGHT as f32)
            .finish();

        let scene = SceneImpl {
            camera,
            width: WIDTH,
            height: HEIGHT,
            world: Box::new(trt_dsl::eval_scene(source)) as _,
            // world: Box::new(final_scene()) as _,
            ray_per_px: RAYS_PER_PX,
        };

        Self(scene)
    }

    pub fn row_color(&self, y: usize) -> Vec<u32> {
        (0..300)
            .map(|x| {
                let Color(r, g, b) = self.0.pixel_color((x, y));
                u32::from_be_bytes([0, r, g, b])
            })
            .collect()
    }

    pub fn pixel_color(&self, x: usize, y: usize) -> u32 {
        let Color(r, g, b) = self.0.pixel_color((x, y));
        u32::from_be_bytes([0, r, g, b])
    }
}

fn final_scene() -> impl Hit {
    let mut boxlist = Vec::<Arc<dyn ParallelHit>>::new();
    let mut boxlist2 = Vec::<Arc<dyn ParallelHit>>::new();

    let white = (0.73, 0.73, 0.73);
    let ground = Arc::new(Lambertian::colored((0.48, 0.83, 0.53)));

    let nb = 20;
    for i in 0..nb {
        for j in 0..nb {
            let w = 100;
            let x0 = (-1000 + i * w) as f32;
            let y0 = 0.;
            let z0 = (-1000 + j * w) as f32;
            let x1 = x0 + w as f32;
            let y1 = 100. * (random::<f32>() + 0.01);
            let z1 = z0 + w as f32;
            boxlist.push(Arc::new(HitBox::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone())));
        }
    }

    let ns = 1000;
    for _ in 0..ns {
        boxlist2.push(Arc::new(Sphere::builder()
            .center((random::<f32>() * 165., random::<f32>() * 165. , random::<f32>() * 165.))
            .radius(10)
            .matte(white)
        ))
    }

    let center = Vec3::new(400., 400., 200.);
    let sphere = || Sphere::builder()
        .center((360, 150, 145))
        .radius(70)
        .dielectric(1.5);
    let pertext = Noise::from_scale(0.1);

    world![
        BVHNode::new(&mut boxlist, 0., 1.),
        RectBuilder
            .x(123..=423)
            .z(147..=412)
            .y(554)
            .diffuse_color((7, 7, 7)),
        MovingSphere::builder()
            .center_from(center)
            .center_to(center + Vec3::new(30, 0, 0))
            .radius(50)
            .matte((0.7, 0.3, 0.1)),
        Sphere::builder()
            .center((260, 150, 45))
            .radius(50)
            .dielectric(1.5),
        Sphere::builder()
            .center((0, 150, 145))
            .radius(50)
            .metallic_fuzzed((0.8, 0.8, 0.9), 10),
        sphere(),
        sphere().constant_medium(0.2, (0.2, 0.4, 0.9)),
        Sphere::builder()
            .radius(5_000)
            .dielectric(1.5)
            .constant_medium(0.0001, (1, 1, 1)),
        // Sphere::builder()
        //     .center((400, 200, 400))
        //     .radius(100)
        //     .matte((0, 1, 0.3)),
        Sphere::builder()
            .center((220, 280, 300))
            .radius(80)
            .material(Lambertian::new(pertext)),
        BVHNode::new(&mut boxlist2, 0., 1.)
            .rotate_y(15.)
            .translate((-100., 270., 395.)),
    ]
}

pub fn cornell_smoke() -> impl Hit {
    let red = (0.65, 0.05, 0.05);
    let white = Arc::new(Lambertian::colored((0.73, 0.73, 0.73)));
    let green = (0.12, 0.45, 0.15);

    let b1 = HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone())
        .rotate_y(-18.)
        .translate((130., 0., 65.));

    let b2 = HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone())
        .rotate_y(15.)
        .translate((265., 0., 295.));

    world![
        RectBuilder.y(0..=555).z(0..=555).x(555).matte(green).flip_normals(),
        RectBuilder.y(0..=555).z(0..=555).x(0).matte(red),
        RectBuilder.x(0..=555).z(0..=555).y(555).material(white.clone()).flip_normals(),
        RectBuilder.x(0..=555).z(0..=555).y(0).material(white.clone()),
        RectBuilder.x(0..=555).y(0..=555).z(555).material(white.clone()).flip_normals(),
        RectBuilder.x(113..=443).z(127..=432).y(554).diffuse_color((7, 7, 7)),
        b1.constant_medium(0.01, (1, 1, 1)),
        b2.constant_medium(0.01, (0, 0, 0)),
    ]
}
