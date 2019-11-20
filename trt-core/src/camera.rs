use crate::prelude::{Vec3, Ray};
use crate::utils::random_in_unit_disk;
use rand::{Rng, thread_rng};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    u: Vec3,
    v: Vec3,
    #[allow(unused)]
    w: Vec3,

    lens_radius: f32,

    time_frame: (f32, f32),
}

impl Camera {
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let mut rng = thread_rng();

        let rd = self.lens_radius * random_in_unit_disk(&mut rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = rng.gen_range(self.time_frame.0, self.time_frame.1);

        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;

        Ray {
            origin: self.origin + offset,
            direction,
            time
        }
    }
}

pub struct CameraBuilder {
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,

    vfov: f32,
    dimensions: (f32, f32),
    aperture: f32,
    focus_dist: f32,

    time_frame: (f32, f32),
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            look_from: Vec3::new(0., 0., 10.),
            look_at: Vec3::splat(0.),
            vup: Vec3::new(0., 1., 0.),

            vfov: 40.0,
            dimensions: (1., 1.),
            aperture: 0.0,
            focus_dist: 10.,

            time_frame: (0., 1.)
        }
    }
}

impl CameraBuilder {
    pub fn finish(self) -> Camera {
        let aspect = self.dimensions.0 / self.dimensions.1.max(1.);

        let lens_radius = self.aperture / 2.;
        let theta = self.vfov * std::f32::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let origin = self.look_from;
        let w = (self.look_from - self.look_at).unit();
        let u = Vec3::cross(self.vup, w).unit();
        let v = Vec3::cross(w, u);

        let lower_left_corner = origin
                              - half_width * self.focus_dist * u
                              - half_height * self.focus_dist * v
                              - self.focus_dist * w;
        let horizontal = 2. * half_width * self.focus_dist * u;
        let vertical = 2. * half_height * self.focus_dist * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,

            u, v, w,

            lens_radius,
            time_frame: self.time_frame,
        }
    }

    pub fn look_from(mut self, from: impl Into<Vec3>) -> Self {
        self.look_from = from.into();
        self
    }

    pub fn look_at(mut self, at: impl Into<Vec3>) -> Self {
        self.look_at = at.into();
        self
    }

    pub fn vup(mut self, vup: impl Into<Vec3>) -> Self {
        self.vup = vup.into();
        self
    }

    pub fn fov(mut self, fov: f32) -> Self {
        self.vfov = fov;
        self
    }

    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        self.dimensions = (width, height);
        self
    }

    pub fn aperture(mut self, aperture: f32) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_distance(mut self, dist: f32) -> Self {
        self.focus_dist = dist;
        self
    }

    pub fn time_frame(mut self, t0: f32, t1: f32) -> Self {
        self.time_frame = (t0, t1);
        self
    }
}
