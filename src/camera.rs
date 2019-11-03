use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    u: Vec3,
    v: Vec3,
    #[allow(unused)]
    w: Vec3,

    lens_radius: f32,

    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32, time0: f32, time1: f32) -> Self {
        let lens_radius = aperture / 2.;
        let theta = vfov * std::f32::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        let lower_left_corner = origin
                              - half_width * focus_dist * u
                              - half_height * focus_dist * v
                              - focus_dist * w;
        let horizontal = 2. * half_width * focus_dist * u;
        let vertical = 2. * half_height * focus_dist * v;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,

            u, v, w,

            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * crate::random_in_unit_disk(rand::thread_rng());
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.time0 + rand::random::<f32>() * (self.time1-self.time0);

        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;

        Ray::new(self.origin + offset, direction)
            .with_time(time)
    }
}
