use rand::{random, thread_rng, Rng};
use rand::seq::SliceRandom;
use indicatif::{ParallelProgressIterator, ProgressStyle, ProgressBar};
use rayon::prelude::*;

use std::sync::Arc;
use std::time;

use trt_core::camera::CameraBuilder;
use trt_core::hit::{Hit, Sphere, MovingSphere, RectBuilder, HitBox, ConstantMedium, BVHNode};
use trt_core::material::{MaterialBuilder, MaterialBuilderExt, Metal, Dielectric, Lambertian, Diffuse, Isotropic};
use trt_core::texture::{Constant, Checker, Noise, Image};
use trt_core::vec3::Vec3;
use trt_core::prelude::ParallelHit;
use trt_core::combine;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;
const RAYS_PER_PX: usize = 50_0;

fn random_scene() -> impl Hit {
    let mut rng = thread_rng();
    let n = 500;
    let mut objects = Vec::<Arc<dyn ParallelHit>>::with_capacity(n);

    let checker = Checker::new(
        Constant::new(Vec3::new(0.9, 0.8, 0.2)),
        Constant::new(Vec3::new(0.3, 0.75, 0.9)),
    );

    objects.push(Arc::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: Lambertian::new(checker),
    }));

    for a in -10..10 {
        for b in -10..10 {
            let choose_mat = random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * random::<f32>(), rng.gen_range(0.2, 5.0), b as f32 + 0.9 * random::<f32>());

            if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.5 {
                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::new(Constant::new(Vec3::random(rng) * Vec3::random(rng)))
                    }));
                } else if choose_mat < 0.90 {
                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::new(
                            (Vec3::random(rng) + Vec3::splat(1.)) * 0.5,
                            0.5 * random::<f32>()
                        )
                    }));
                } else {
                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::new(1.5),
                    }));
                };
            }
        }
    }

    objects.push(Arc::new(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        material: Dielectric::new(1.5),
    }));

    objects.push(Arc::new(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        material: Lambertian::new(Constant::new(Vec3::new(0.4, 0.2, 0.1))),
    }));

    objects.push(Arc::new(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        material: Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    }));

    objects.push(Arc::new(
        RectBuilder
            .x(-5..=5)
            .z(-50..=5)
            .y(20)
            .metallic((1, 1, 1))
    ));

    BVHNode::new(&mut objects, 0., 1.)
}

fn two_perlin_spheres() -> impl Hit {
    let pertext = Noise::from_scale(5.);

    let earth_img = Image::load("./assets/earthmap.jpg")
        .expect("Failed to load image");

    combine![
        Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: Lambertian::new(pertext),
        },
        Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: Lambertian::new(earth_img)
        },
        RectBuilder
            .x(-5..=5)
            .z(-50..=5)
            .y(20)
            .diffuse_color((5, 5, 5))
    ]
}

fn simple_light() -> impl Hit {
    let pertext = || Noise::from_scale(4.);

    let earth_img = Image::load("./assets/earthmap.jpg")
        .expect("Failed to load image");

    combine![
        Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: Lambertian::new(pertext())
        },
        Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: Lambertian::new(earth_img)
        },
        Sphere {
            center: Vec3::new(0., 7., 0.),
            radius: 2.,
            material: Diffuse::colored((4, 4, 4)),
        },
        RectBuilder
            .x(3..=5)
            .y(1..=3)
            .z(-2)
            .diffuse_color((4, 4, 4))
    ]
}

fn cornell_box() -> impl Hit {
    let red = Lambertian::new(Constant::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = || Lambertian::new(Constant::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(Constant::new(Vec3::new(0.12, 0.45, 0.15)));

    let oreo_img = Image::load("./assets/oreo.jpg")
        .expect("Failed to load image");

    let img_text = Arc::new(Lambertian::new(oreo_img));

    combine![
        RectBuilder.y(0..=555).z(0..=555).x(555).material(green).flip_normals(),
        RectBuilder.y(0..=555).z(0..=555).x(0).material(red),
        RectBuilder.x(0..=555).z(0..=555).y(555).material(white()).flip_normals(),
        RectBuilder.x(0..=555).z(0..=555).y(0).material(white()),
        RectBuilder.x(0..=555).y(0..=555).z(555).material(white()).flip_normals(),
        RectBuilder.x(113..=443).z(127..=432).y(554).diffuse_color((15, 15, 15)),
        HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), img_text.clone())
            .rotate_y(-18.)
            .translate((130., 0., 65.)),
        HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), img_text)
            .rotate_y(15.)
            .translate((265., 0., 295.)),
    ]
}

fn cornell_smoke() -> impl Hit {
    let red = (0.65, 0.05, 0.05);
    let white = Arc::new(Lambertian::new(Constant::new(Vec3::new(0.73, 0.73, 0.73))));
    let green = (0.12, 0.45, 0.15);

    let b1 = HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone())
        .rotate_y(-18.)
        .translate((130., 0., 65.));

    let b2 = HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone())
        .rotate_y(15.)
        .translate((265., 0., 295.));

    combine![
        RectBuilder.y(0..=555).z(0..=555).x(555).matte(green).flip_normals(),
        RectBuilder.y(0..=555).z(0..=555).x(0).matte(red),
        RectBuilder.x(0..=555).z(0..=555).y(555).material(white.clone()).flip_normals(),
        RectBuilder.x(0..=555).z(0..=555).y(0).material(white.clone()),
        RectBuilder.x(0..=555).y(0..=555).z(555).material(white.clone()).flip_normals(),
        RectBuilder.x(113..=443).z(127..=432).y(554).diffuse_color((7, 7, 7)),
        ConstantMedium {
            boundary: b1,
            density: 0.01,
            phase_function: Isotropic::new(Constant::new(Vec3::new(1., 1., 1.))),
        },
        ConstantMedium {
            boundary: b2,
            density: 0.01,
            phase_function: Isotropic::new(Constant::new(Vec3::new(0., 0., 0.))),
        },
    ]
}

fn final_scene() -> impl Hit {
    let mut boxlist = Vec::<Arc<dyn ParallelHit>>::new();
    let mut boxlist2 = Vec::<Arc<dyn ParallelHit>>::new();

    let white = || Lambertian::new(Constant::new(Vec3::new(0.73, 0.73, 0.73)));
    let ground = Arc::new(Lambertian::new(Constant::new(Vec3::new(0.48, 0.83, 0.53))));

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

    let globibot_img = Image::load("./assets/globibot.png")
        .expect("Failed to load image");

    let ns = 1000;
    for _ in 0..ns {
        boxlist2.push(Arc::new(Sphere {
            center: Vec3::new(165. * random::<f32>(), 165. * random::<f32>(), 165. * random::<f32>()),
            radius: 10.,
            material: white(),
        }))
    }

    let center = Vec3::new(400., 400., 200.);
    let boundary = || Sphere {
        center: Vec3::new(360., 150., 145.),
        radius: 70.,
        material: Dielectric::new(1.5),
    };
    let pertext = Noise::from_scale(0.1);

    combine![
        BVHNode::new(&mut boxlist, 0., 1.),
        RectBuilder
            .x(123..=423)
            .z(147..=412)
            .y(554)
            .diffuse_color((7, 7, 7)),
        MovingSphere {
            center0: center,
            center1: center + Vec3::new(30., 0., 0.),
            time0: 0.,
            time1: 1.,
            radius: 50.,
            material: Lambertian::new(Constant::new(Vec3::new(0.7, 0.3, 0.1)))
        },
        Sphere {
            center: Vec3::new(260., 150., 45.),
            radius: 50.,
            material: Dielectric::new(1.5),
        },
        Sphere {
            center: Vec3::new(0., 150., 145.),
            radius: 50.,
            material: Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.),
        },
        boundary(),
        ConstantMedium {
            boundary: boundary(),
            density: 0.2,
            phase_function: Isotropic::new(Constant::new(Vec3::new(0.2, 0.4, 0.9)))
        },
        ConstantMedium {
            boundary: Sphere {
                center: Vec3::new(0., 0., 0.),
                radius: 5000.,
                material: Dielectric::new(1.5),
            },
            density: 0.0001,
            phase_function: Isotropic::new(Constant::new(Vec3::new(1.0, 1.0, 1.0))),
        },
        Sphere {
            center: Vec3::new(400., 200., 400.),
            radius: 100.,
            material: Lambertian::new(globibot_img),
        },
        Sphere {
            center: Vec3::new(220., 280., 300.),
            radius: 80.,
            material: Lambertian::new(pertext),
        },
        BVHNode::new(&mut boxlist2, 0., 1.)
            .rotate_y(15.)
            .translate((-100., 270., 395.)),
    ]
}

fn run() -> image::RgbImage {
    use std::time::Instant;

    let now = Instant::now();

    let camera = CameraBuilder::default()
        .look_from((478., 278., -600.))
        .look_at((278., 278., 0.))
        .dimensions(WIDTH as f32, HEIGHT as f32)
        .finish();

    let world = final_scene();

    let progress = ProgressBar::new((WIDTH * HEIGHT) as u64)
        .with_style(ProgressStyle::default_bar().template("{pos:>7}/{len:7} {bar:40.cyan/yellow} - [{elapsed_precise}] [{eta_precise}]"));

    let bytes = (0..HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| (0..WIDTH).into_par_iter().map(move |i| (i, j)))
        .map(|(i, j)| {
            trt_core::render(&world, (i, j), (WIDTH, HEIGHT), &camera, RAYS_PER_PX)
        })
        .progress_with(progress)
        .flat_map(|trt_core::Color(r, g, b)| {
            use rayon::iter::once;

            once(r).chain(once(g)).chain(once(b))
        })
        .collect::<Vec<_>>();

    println!("Elapsed: {:?}", now.elapsed());

    image::RgbImage::from_vec(WIDTH as u32, HEIGHT as u32, bytes)
        .expect("Image and buffer dimension mismatch")
}

fn main() {
    let image = run();

    let epoch_secs = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Invalid times")
        .as_secs();

    let path = format!("./generated/{}.png", epoch_secs);

    image.save(path)
        .expect("Failed to save image")
}
