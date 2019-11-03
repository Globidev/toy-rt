#![feature(test)]
use rand::random;
use indicatif::{ParallelProgressIterator, ProgressStyle, ProgressBar};
use rayon::prelude::*;

use std::sync::Arc;
use std::time;

mod prelude;
mod hit;
mod material;
mod texture;
mod camera;
mod ray;
mod vec3;
mod aabb;
mod perlin;

use camera::Camera;
use hit::{Hit, HitList, Sphere, MovingSphere, XYRect, XZRect, YZRect, FlipNormals, HitBox, Translate, RotateY, ConstantMedium, BVHNode};
use material::{Metal, Dielectric, Lambertian, DiffuseLight, Isotropic};
use texture::{ConstantTexture, CheckerTexture, NoiseTexture, ImageTexture};
use ray::Ray;
use vec3::Vec3;
use perlin::Perlin;
use prelude::ParallelHit;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;
const RAYS_PER_PX: usize = 1_000;

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

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::random() - Vec3::splat(1.);
        if Vec3::dot(p, p) < 1.0 {
            break p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), 0.) - Vec3::new(1., 1., 0.);
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

pub fn ffmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

pub fn ffmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn random_scene() -> impl Hit {
    let n = 500;
    let mut objects = Vec::<Box<dyn ParallelHit>>::with_capacity(n);

    let checker = CheckerTexture {
        odd: ConstantTexture { color: Vec3::new(0.2, 0.3, 0.1) },
        even: ConstantTexture { color: Vec3::new(0.9, 0.9, 0.9) },
    };

    objects.push(Box::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: Lambertian {
            albedo: checker
        }
    }));

    for a in -10..10 {
        for b in -10..10 {
            let choose_mat = random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>());

            if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.8 {
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian {
                            albedo: ConstantTexture { color: Vec3::random() * Vec3::random() }
                        }
                    }));
                } else if choose_mat < 0.95 {
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::new(
                            (Vec3::random() + Vec3::splat(1.)) * 0.5,
                            0.5 * random::<f32>()
                        )
                    }));
                } else {
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric { ref_idx: 1.5 }
                    }));
                };
            }
        }
    }

    objects.push(Box::new(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        material: Dielectric {
            ref_idx: 1.5
        }
    }));

    objects.push(Box::new(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        material: Lambertian {
            albedo: ConstantTexture { color: Vec3::new(0.4, 0.2, 0.1) }
        }
    }));

    objects.push(Box::new(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }
    }));

    HitList::new_dyn(objects)
}

fn two_spheres() -> impl Hit {
    let checker = || CheckerTexture {
        odd: ConstantTexture { color: Vec3::new(0.2, 0.3, 0.1) },
        even: ConstantTexture { color: Vec3::new(0.9, 0.9, 0.9) },
    };

    HitList::new_dyn(vec![
        Box::new(Sphere {
            center: Vec3::new(0., -10., 0.),
            radius: 10.,
            material: Lambertian {
                albedo: checker()
            }
        }),
        Box::new(Sphere {
            center: Vec3::new(0., 10., 0.),
            radius: 10.,
            material: Lambertian {
                albedo: checker()
            }
        }),
    ])
}

fn two_perlin_spheres() -> impl Hit {
    let pertext = || NoiseTexture { perlin: Perlin::new(), scale: 5. };

    let image = image::io::Reader::open("./earthmap.jpg")
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let image = match image {
        image::DynamicImage::ImageRgb8(rgb) => rgb,
        _ => panic!("Wrong format")
    };

    HitList::new_dyn(vec![
        Box::new(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: Lambertian {
                albedo: pertext()
            }
        }),
        Box::new(Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: Lambertian {
                albedo: ImageTexture { image }
            }
        }),
    ])
}

fn simple_light() -> impl Hit {
    let pertext = || NoiseTexture { perlin: Perlin::new(), scale: 4. };

    let image = image::io::Reader::open("./earthmap.jpg")
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let image = match image {
        image::DynamicImage::ImageRgb8(rgb) => rgb,
        _ => panic!("Wrong format")
    };

    HitList::new_dyn(vec![
        Box::new(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: Lambertian {
                albedo: pertext()
            }
        }),
        Box::new(Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: Lambertian {
                albedo: ImageTexture { image }
            }
        }),
        Box::new(Sphere {
            center: Vec3::new(0., 7., 0.),
            radius: 2.,
            material: DiffuseLight {
                emit: ConstantTexture { color: Vec3::new(4., 4., 4.) }
            }
        }),
        Box::new(XYRect {
            x0: 3.,
            x1: 5.,
            y0: 1.,
            y1: 3.,
            k: -2.,
            material: DiffuseLight {
                emit: ConstantTexture { color: Vec3::new(4., 4., 4.) }
            }
        }),
    ])
}

fn cornell_box() -> impl Hit {
    let red = Lambertian { albedo: ConstantTexture { color: Vec3::new(0.65, 0.05, 0.05) } };
    let white = || Lambertian { albedo: ConstantTexture { color: Vec3::new(0.73, 0.73, 0.73) } };
    let green = Lambertian { albedo: ConstantTexture { color: Vec3::new(0.12, 0.45, 0.15) } };
    let light = DiffuseLight { emit: ConstantTexture { color: Vec3::new(15.0, 15.0, 15.0) } };
    // let light = Arc::new(DiffuseLight { emit: Box::new(ConstantTexture { color: Vec3::new(7.0, 7.0, 7.0) }) });

    let image = image::io::Reader::open("./oreo.jpg")
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let image = match image {
        image::DynamicImage::ImageRgb8(rgb) => rgb,
        _ => panic!("Wrong format")
    };

    let img_text = Arc::new(Lambertian { albedo: ImageTexture { image: image.clone() } });

    HitList::new_dyn(vec![
        Box::new(FlipNormals { hittable: YZRect { y0: 0., y1: 555., z0: 0., z1: 555., k: 555., material: green } }),
        Box::new(YZRect { y0: 0., y1: 555., z0: 0., z1: 555., k: 0., material: red }),
        Box::new(XZRect { x0: 113., x1: 443., z0: 127., z1: 432., k: 554., material: light }),
        Box::new(FlipNormals { hittable: XZRect { x0: 0., x1: 555., z0: 0., z1: 555., k: 555., material: white() } }),
        Box::new(XZRect { x0: 0., x1: 555., z0: 0., z1: 555., k: 0., material: white() }),
        Box::new(FlipNormals { hittable: XYRect { x0: 0., x1: 555., y0: 0., y1: 555., k: 555., material: white() } }),
        // Box::new(HitBox::new(Vec3([130., 0., 65.]), Vec3([295., 165., 230.]), white)),
        // Box::new(HitBox::new(Vec3([265., 0., 295.]), Vec3([430., 330., 460.]), white)),
        Box::new(Translate {
            hittable: RotateY::new(
                HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), img_text.clone()),
                -18.
            ),
            offset: Vec3::new(130., 0., 65.)
        }),
        Box::new(Translate {
            hittable: RotateY::new(
                HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), img_text),
                15.
            ),
            offset: Vec3::new(265., 0., 295.)
        }),
    ])
}

fn cornell_smoke() -> impl Hit {
    let red = Lambertian { albedo: ConstantTexture { color: Vec3::new(0.65, 0.05, 0.05) } };
    let white = Arc::new(Lambertian { albedo: ConstantTexture { color: Vec3::new(0.73, 0.73, 0.73) } });
    let green = Lambertian { albedo: ConstantTexture { color: Vec3::new(0.12, 0.45, 0.15) } };
    let light = DiffuseLight { emit: ConstantTexture { color: Vec3::new(7.0, 7.0, 7.0) } };
    // let light = Box::new(DiffuseLight { emit: Box::new(ConstantTexture { color: Vec3([15.0, 15.0, 15.0]) }) });

    let b1 = Translate {
        hittable: RotateY::new(
            HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone()),
            -18.
        ),
        offset: Vec3::new(130., 0., 65.)
    };

    let b2 = Translate {
        hittable: RotateY::new(
            HitBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone()),
            15.
        ),
        offset: Vec3::new(265., 0., 295.)
    };

    HitList::new_dyn(vec![
        Box::new(FlipNormals {
            hittable: YZRect { y0: 0., y1: 555., z0: 0., z1: 555., k: 555., material: green }
        }),
        Box::new(YZRect { y0: 0., y1: 555., z0: 0., z1: 555., k: 0., material: red }),
        Box::new(XZRect { x0: 113., x1: 443., z0: 127., z1: 432., k: 554., material: light }),
        Box::new(FlipNormals {
            hittable: XZRect { x0: 0., x1: 555., z0: 0., z1: 555., k: 555., material: white.clone() }
        }),
        Box::new(XZRect { x0: 0., x1: 555., z0: 0., z1: 555., k: 0., material: white.clone() }),
        Box::new(FlipNormals {
            hittable: XYRect { x0: 0., x1: 555., y0: 0., y1: 555., k: 555., material: white }
        }),
        Box::new(ConstantMedium {
            boundary: b1,
            density: 0.01,
            phase_function: Isotropic {
                albedo: ConstantTexture { color: Vec3::new(1., 1., 1.) }
            }
        }),
        Box::new(ConstantMedium {
            boundary: b2,
            density: 0.01,
            phase_function: Isotropic {
                albedo: ConstantTexture { color: Vec3::new(0., 0., 0.) }
            }
        }),
    ])
}

fn final_scene() -> impl Hit {
    let mut list = Vec::<Box<dyn ParallelHit>>::new();
    let mut boxlist = Vec::<Arc<dyn ParallelHit>>::new();
    let mut boxlist2 = Vec::<Arc<dyn ParallelHit>>::new();

    let white = || Lambertian { albedo: ConstantTexture { color: Vec3::new(0.73, 0.73, 0.73) } };
    let ground = Arc::new(Lambertian { albedo: ConstantTexture { color: Vec3::new(0.48, 0.83, 0.53) } });

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

    list.push(Box::new(BVHNode::new(&mut boxlist, 0., 1.)));

    let light = DiffuseLight { emit: ConstantTexture { color: Vec3::new(7., 7., 7.) } };
    list.push(Box::new(XZRect { x0: 123., x1: 423., z0: 147., z1: 412., k: 554., material: light }));

    let center = Vec3::new(400., 400., 200.);
    list.push(Box::new(MovingSphere {
        center0: center,
        center1: center + Vec3::new(30., 0., 0.),
        time0: 0.,
        time1: 1.,
        radius: 50.,
        material: Lambertian {
            albedo: ConstantTexture { color: Vec3::new(0.7, 0.3, 0.1) }
        },
    }));
    list.push(Box::new(Sphere {
        center: Vec3::new(260., 150., 45.),
        radius: 50.,
        material: Dielectric { ref_idx: 1.5 },
    }));
    list.push(Box::new(Sphere {
        center: Vec3::new(0., 150., 145.),
        radius: 50.,
        material: Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.),
    }));

    let boundary = || Sphere {
        center: Vec3::new(360., 150., 145.),
        radius: 70.,
        material: Dielectric { ref_idx: 1.5 },
    };
    list.push(Box::new(boundary()));
    list.push(Box::new(ConstantMedium {
        boundary: boundary(),
        density: 0.2,
        phase_function: Isotropic {
            albedo: ConstantTexture { color: Vec3::new(0.2, 0.4, 0.9) },
        },
    }));
    list.push(Box::new(ConstantMedium {
        boundary: Sphere {
            center: Vec3::new(0., 0., 0.),
            radius: 5000.,
            material: Dielectric { ref_idx: 1.5 },
        },
        density: 0.0001,
        phase_function: Isotropic {
            albedo: ConstantTexture { color: Vec3::new(1.0, 1.0, 1.0) },
        },
    }));

    let image = image::io::Reader::open("./earthmap.jpg")
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");
    let image = match image {
        image::DynamicImage::ImageRgb8(rgb) => rgb,
        _i => panic!("Wrong format")
    };
    list.push(Box::new(Sphere {
        center: Vec3::new(400., 200., 400.),
        radius: 100.,
        material: Lambertian { albedo: ImageTexture { image } },
    }));
    let pertext = NoiseTexture { perlin: Perlin::new(), scale: 0.1 };
    list.push(Box::new(Sphere {
        center: Vec3::new(220., 280., 300.),
        radius: 80.,
        material: Lambertian { albedo: pertext },
    }));

    let ns = 1000;
    for _ in 0..ns {
        boxlist2.push(Arc::new(Sphere {
            center: Vec3::new(165. * random::<f32>(), 165. * random::<f32>(), 165. * random::<f32>()),
            radius: 10.,
            material: white(),
        }))
    }

    list.push(Box::new(Translate {
        hittable: RotateY::new(
            BVHNode::new(&mut boxlist2, 0., 1.),
            15.,
        ),
        offset: Vec3::new(-100., 270., 395.),
    }));

    HitList::new_dyn(list)
}

fn clamp(x: f32) -> u8 {
    let clamped = if x > 255. { 255. } else { x };
    clamped as u8
}


fn run() -> image::RgbImage {
    use std::time::Instant;

    let now = Instant::now();

    // let look_from = Vec3::new(278., 278., -800.);
    let look_from = Vec3::new(478., 278., -600.);
    let look_at = Vec3::new(278., 278., 0.);

    // let look_from = Vec3::new(13., 10., 3.);
    // let look_at = Vec3::new(0., 0., 0.);

    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    // let vfov = 20.0;

    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0., 1., 0.),
        vfov,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        dist_to_focus,
        0.0, 1.0,
    );

    // let world = cornell_box();
    // let world = cornell_smoke();
    let world = final_scene();
    // let world = simple_light();

    let progress = ProgressBar::new((WIDTH * HEIGHT) as u64)
        .with_style(ProgressStyle::default_bar().template("{pos:>7}/{len:7} {bar:40.cyan/yellow} - [{elapsed_precise}] [{eta_precise}]"));

    let bytes = (0..HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| (0..WIDTH).into_par_iter().map(move |i| (i, j)))
        .map(|(i, j)| {
            let mut col = Vec3::splat(0.);
            for _s in 0..RAYS_PER_PX {
                let u = (i as f32 + random::<f32>()) / WIDTH as f32;
                let v = (j as f32 + random::<f32>()) / HEIGHT as f32;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= RAYS_PER_PX as f32;
            col = col.sqrt();

            col

        })
        .progress_with(progress)
        .flat_map(|col| {
            let r = 255.99 * col.r();
            let g = 255.99 * col.g();
            let b = 255.99 * col.b();

            rayon::iter::once(clamp(r))
                .chain(rayon::iter::once(clamp(g)))
                .chain(rayon::iter::once(clamp(b)))
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
