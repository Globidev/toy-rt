use crate::prelude::{Material, Hit, AABB, HitRecord, Ray, Vec3, Asf32};
use crate::material::MaterialBuilder;
use crate::utils::sphere_uv;

pub struct Sphere<Mat> {
    center: Vec3,
    radius: f32,
    material: Mat,
}

pub struct UnboundedMat;

impl Sphere<UnboundedMat> {
    pub fn builder() -> SphereBuilder {
        SphereBuilder::default()
    }
}

impl<Mat: Material> Hit for Sphere<Mat> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let disc_sqrt = discriminant.sqrt();
            for &solution in &[(-b - disc_sqrt) / a, (-b + disc_sqrt) / a] {
                if solution < t_max && solution > t_min {
                    let p = ray.point_at_parameter(solution);
                    let normal = (p - self.center) / self.radius;
                    let (u, v) = sphere_uv((p - self.center) / self.radius);
                    return Some(HitRecord { t: solution, p, normal, mat: &self.material, u, v })
                }
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let radius = self.radius;
        Some(AABB {
            min: self.center - Vec3::splat(radius),
            max: self.center + Vec3::splat(radius),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct SphereBuilder {
    center: Option<Vec3>,
    radius: Option<f32>,
}

impl SphereBuilder {
    pub fn center(mut self, center: impl Into<Vec3>) -> Self {
        self.center = Some(center.into());
        self
    }

    pub fn radius(mut self, radius: impl Asf32) -> Self {
        self.radius = Some(radius.as_());
        self
    }
}

impl<Mat> MaterialBuilder<Mat> for SphereBuilder {
    type Finished = Sphere<Mat>;

    fn material(self, material: Mat) -> Self::Finished {
        Sphere {
            center: self.center.unwrap_or_default(),
            radius: self.radius.unwrap_or_default(),
            material,
        }
    }
}
