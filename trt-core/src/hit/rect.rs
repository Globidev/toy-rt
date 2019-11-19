use crate::prelude::{Material, Hit, AABB, HitRecord, Ray, Vec3, Dimension, X, Y, Z, Asf32};
use crate::material::MaterialBuilder;
use std::{ops::RangeInclusive, marker::PhantomData};

type DimRange = RangeInclusive<f32>;

pub struct Rect<D1, D2, D3, Mat> {
    d1_range: DimRange,
    d2_range: DimRange,
    d3: f32,
    material: Mat,
    tag: PhantomData<(D1, D2, D3)>,
}

impl<D1, D2, D3, Mat> Hit for Rect<D1, D2, D3, Mat>
where
    D1: Dimension,
    D2: Dimension,
    D3: Dimension,
    Mat: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.d3 - ray.origin.get::<D3>()) / ray.direction.get::<D3>();

        if t < t_min || t > t_max {
            return None
        }

        let d1 = ray.origin.get::<D1>() + t * ray.direction.get::<D1>();
        let d2 = ray.origin.get::<D2>() + t * ray.direction.get::<D2>();

        if !self.d1_range.contains(&d1) || !self.d2_range.contains(&d2) {
            return None
        }

        let (d1_0, d1_1) = (self.d1_range.start(), self.d1_range.end());
        let (d2_0, d2_1) = (self.d2_range.start(), self.d2_range.end());

        let u = (d1 - d1_0) / (d1_1 - d1_0);
        let v = (d2 - d2_0) / (d2_1 - d2_0);

        Some(HitRecord {
            u, v, t,
            mat: &self.material,
            p: ray.point_at_parameter(t),
            normal: Vec3::splat(0.).set::<D3>(1.),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let (&d1_0, &d1_1) = (self.d1_range.start(), self.d1_range.end());
        let (&d2_0, &d2_1) = (self.d2_range.start(), self.d2_range.end());

        let min = Vec3::splat(self.d3 - 0.0001).set::<D1>(d1_0).set::<D2>(d2_0);
        let max = Vec3::splat(self.d3 + 0.0001).set::<D1>(d1_1).set::<D2>(d2_1);

        Some(AABB { min, max })
    }
}

pub struct RectBuilder;

macro_rules! builder_method {
    ($name:ident, $tag:ty) => {
        pub fn $name(self, range: RangeInclusive<impl Asf32>) -> OneBoundedRectBuilder<$tag> {
            OneBoundedRectBuilder {
                range: range.start().as_()..=range.end().as_(),
                tag: PhantomData
            }
        }
    }
}

impl RectBuilder {
    builder_method!(x, X);
    builder_method!(y, Y);
    builder_method!(z, Z);
}

pub struct OneBoundedRectBuilder<D> {
    range: DimRange,
    tag: PhantomData<D>,
}

macro_rules! one_bound_builder_method {
    ($name:ident, $tag1:ty, $tag2:ty) => {
        pub fn $name(self, range: RangeInclusive<impl Asf32>) -> TwoBoundedRectBuilder<$tag1, $tag2> {
            TwoBoundedRectBuilder {
                d1_range: self.range,
                d2_range: range.start().as_()..=range.end().as_(),
                tag: PhantomData
            }
        }
    }
}

impl OneBoundedRectBuilder<X> {
    one_bound_builder_method!(y, X, Y);
    one_bound_builder_method!(z, X, Z);
}

impl OneBoundedRectBuilder<Y> {
    one_bound_builder_method!(x, Y, X);
    one_bound_builder_method!(z, Y, Z);
}

impl OneBoundedRectBuilder<Z> {
    one_bound_builder_method!(x, Z, X);
    one_bound_builder_method!(y, Z, Y);
}

pub struct TwoBoundedRectBuilder<D1, D2> {
    d1_range: DimRange,
    d2_range: DimRange,
    tag: PhantomData<(D1, D2)>,
}

macro_rules! two_bound_builder_method {
    ($name:ident, $tag1:ty, $tag2:ty, $tag3:ty) => {
        pub fn $name(self, $name: impl Asf32) -> ThreeBoundedRectBuilder<$tag1, $tag2, $tag3> {
            ThreeBoundedRectBuilder {
                d1_range: self.d1_range,
                d2_range: self.d2_range,
                d3: $name.as_(),
                tag: PhantomData
            }
        }
    }
}

impl TwoBoundedRectBuilder<X, Y> {
    two_bound_builder_method!(z, X, Y, Z);
}

impl TwoBoundedRectBuilder<X, Z> {
    two_bound_builder_method!(y, X, Z, Y);
}

impl TwoBoundedRectBuilder<Y, Z> {
    two_bound_builder_method!(x, Y, Z, X);
}

pub struct ThreeBoundedRectBuilder<D1, D2, D3> {
    d1_range: DimRange,
    d2_range: DimRange,
    d3: f32,
    tag: PhantomData<(D1, D2, D3)>,
}

impl<D1, D2, D3, Mat> MaterialBuilder<Mat> for ThreeBoundedRectBuilder<D1, D2, D3> {
    type Finished = Rect<D1, D2, D3, Mat>;

    fn material(self, material: Mat) -> Self::Finished {
        Rect {
            d1_range: self.d1_range,
            d2_range: self.d2_range,
            d3: self.d3,
            material,
            tag: PhantomData,
        }
    }
}
