use crate::prelude::{Material, Hit, AABB, HitRecord, Ray, Vec3, Asf32};
use crate::material::MaterialBuilder;

pub struct MovingSphere<T> {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: T,
}

impl<T: Material> MovingSphere<T> {
    fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

pub struct UnboundedMat;

impl MovingSphere<UnboundedMat> {
    pub fn builder() -> MovingSphereBuilder {
        MovingSphereBuilder::default()
    }
}

impl<T: Material> Hit for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.center(ray.time);
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let disc_sqrt = discriminant.sqrt();

            for &solution in &[(-b - disc_sqrt) / a, (-b + disc_sqrt) / a] {
                if solution < t_max && solution > t_min {
                    let p = ray.point_at_parameter(solution);
                    let normal = (p - self.center(ray.time)) / self.radius;
                    return Some(HitRecord { t: solution, p, normal, mat: &self.material, u: 0., v: 0. })
                }
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let radius = self.radius;

        let box0 = AABB {
            min: self.center(t0) - Vec3::splat(radius),
            max: self.center(t0) + Vec3::splat(radius),
        };
        let box1 = AABB {
            min: self.center(t1) - Vec3::splat(radius),
            max: self.center(t1) + Vec3::splat(radius),
        };

        Some(AABB::surrounding_box(box0, box1))
    }
}

#[derive(Default)]
pub struct MovingSphereBuilder {
    center0: Option<Vec3>,
    center1: Option<Vec3>,
    time_frame: Option<(f32, f32)>,
    radius: Option<f32>,
}

impl MovingSphereBuilder {
    pub fn center_from(mut self, center: impl Into<Vec3>) -> Self {
        self.center0 = Some(center.into());
        self
    }

    pub fn center_to(mut self, center: impl Into<Vec3>) -> Self {
        self.center1 = Some(center.into());
        self
    }

    pub fn radius(mut self, radius: impl Asf32) -> Self {
        self.radius = Some(radius.as_());
        self
    }

    pub fn time_frame(mut self, t0: f32, t1: f32) -> Self {
        self.time_frame = Some((t0, t1));
        self
    }
}

impl<Mat> MaterialBuilder<Mat> for MovingSphereBuilder {
    type Finished = MovingSphere<Mat>;

    fn material(self, material: Mat) -> Self::Finished {
        let (time0, time1) = self.time_frame.unwrap_or((0., 1.));

        MovingSphere {
            center0: self.center0.unwrap_or_default(),
            center1: self.center1.unwrap_or_default(),
            radius: self.radius.unwrap_or_default(),
            time0, time1, material,
        }
    }
}
