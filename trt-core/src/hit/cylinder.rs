use crate::prelude::{Material, Hit, AABB, HitRecord, Ray, Vec3, Asf32};
use crate::{utils::cylinder_uv, material::MaterialBuilder};

pub struct Cylinder<Mat> {
    base: Vec3,
    height: f32,
    radius: f32,
    material: Mat,
}

pub struct UnboundedMat;

impl Cylinder<UnboundedMat> {
    pub fn builder() -> CylinderBuilder {
        CylinderBuilder::default()
    }
}

impl<Mat: Material> Hit for Cylinder<Mat> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.base;

        let a = ray.direction.x() * ray.direction.x() + ray.direction.z() * ray.direction.z();
        let b = oc.x() * ray.direction.x() + oc.z() * ray.direction.z();
        let c = (oc.x() * oc.x() + oc.z() * oc.z()) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let disc_sqrt = discriminant.sqrt();

            let mut near = (-b - disc_sqrt) / a;
            let mut far = (-b + disc_sqrt) / a;

            if near > far {
                std::mem::swap(&mut near, &mut far);
            }

            let ynear = oc.y() + near * ray.direction.y();
            let yfar = oc.y() + far * ray.direction.y();

            let ycap = (self.height, 0.);
            let cap = ((ycap.0 - oc.y()) / ray.direction.y(), (ycap.1 - oc.y()) / ray.direction.y());

            let mut capped = false;
            let mut cap_neg = false;
            if ynear < ycap.1 {
                near = cap.1;
                capped = true;
                cap_neg = true;
            } else if ynear > ycap.0 {
                near = cap.0;
                capped = true;
            }

            if yfar < ycap.1 {
                far = cap.1
            } else if yfar > ycap.0 {
                far = cap.0
            }

            if far > near && near < t_max && near > t_min {
                let t = near;
                let p = ray.point_at_parameter(t);
                let normal = if capped {
                    Vec3::new(0, if cap_neg { -1. } else { 1. }, 0)
                } else {
                    let centered = p - self.base;
                    Vec3::new(centered.x(), 0., centered.z()) / self.radius
                };
                let (u, v) = cylinder_uv(p);
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    mat: &self.material,
                    u, v
                })
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let r = self.radius;
        Some(AABB {
            min: self.base - Vec3::new(r, 0, r),
            max: self.base + Vec3::new(r, self.height, r),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct CylinderBuilder {
    base: Option<Vec3>,
    height: Option<f32>,
    radius: Option<f32>,
}

impl CylinderBuilder {
    pub fn base(mut self, v: impl Into<Vec3>) -> Self {
        self.base = Some(v.into());
        self
    }

    pub fn height(mut self, v: impl Asf32) -> Self {
        self.height = Some(v.as_());
        self
    }

    pub fn radius(mut self, radius: impl Asf32) -> Self {
        self.radius = Some(radius.as_());
        self
    }
}

impl<Mat> MaterialBuilder<Mat> for CylinderBuilder {
    type Finished = Cylinder<Mat>;

    fn material(self, material: Mat) -> Self::Finished {
        Cylinder {
            base: self.base.unwrap_or_default(),
            height: self.height.unwrap_or_default(),
            radius: self.radius.unwrap_or_default(),
            material,
        }
    }
}
