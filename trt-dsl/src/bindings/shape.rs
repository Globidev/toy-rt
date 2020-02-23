use trt_core::{
    hit::{RectBuilder, Sphere, HitBox, BVHNode},
    prelude::*,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue, TryFromObject},
};

use super::{float::FloatLike, SharedHit, material::PyMaterial, vec3::PyVec3};
use std::rc::Rc;
use rpy::{obj::objlist::PyListRef, pyobject::PyRef};

use futures::prelude::*;
use crate::future::PyFuture;

#[rpy::pyclass(name = "Shape")]
#[derive(Debug)]
pub struct PyShape(SharedHit);

impl PyShape {
    pub fn shared_hit(&self) -> SharedHit {
        self.0.clone()
    }
}

impl PyValue for PyShape {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Shape")
    }
}

#[rpy::pyimpl]
impl PyShape {
    #[pyclassmethod]
    fn sphere(_cls: PyClassRef, center: PyVec3, radius: f32, material: PyMaterial, _vm: &rpy::VirtualMachine) -> Self {
        let sphere_fut = material.shared_material()
            .map(move |mat| {
                let sphere = Sphere::builder()
                    .radius(radius)
                    .center(center.into_vec())
                    .material(mat);

                Rc::new(sphere) as _
            });

        Self(SharedHit(sphere_fut))
    }

    #[pyclassmethod]
    fn xy_rect(_cls: PyClassRef, x: (f32, f32), y: (f32, f32), z: f32, material: PyMaterial) -> Self {
        let rect_fut = material.shared_material()
            .map(move |mat| {
                let rect = RectBuilder.x(x.0..=x.1).y(y.0..=y.1).z(z).material(mat);
                Rc::new(rect) as _
            });

        Self(SharedHit(rect_fut))
    }

    #[pyclassmethod]
    fn xz_rect(_cls: PyClassRef, x: (f32, f32), z: (f32, f32), y: f32, material: PyMaterial) -> Self {
        let rect_fut = material.shared_material()
            .map(move |mat| {
                let rect = RectBuilder.x(x.0..=x.1).z(z.0..=z.1).y(y).material(mat);
                Rc::new(rect) as _
            });

        Self(SharedHit(rect_fut))
    }

    #[pyclassmethod]
    fn yz_rect(_cls: PyClassRef, y: (f32, f32), z: (f32, f32), x: f32, material: PyMaterial) -> Self {
        let rect_fut = material.shared_material()
            .map(move |mat| {
                let rect = RectBuilder.y(y.0..=y.1).z(z.0..=z.1).x(x).material(mat);
                Rc::new(rect) as _
            });

        Self(SharedHit(rect_fut))
    }

    #[pyclassmethod]
    fn hitbox(_cls: PyClassRef, min: PyVec3, max: PyVec3, material: PyMaterial) -> Self {
        let hitbox_fut = material.shared_material()
            .map(move |mat| {
                let hitbox = HitBox::new(
                    min.into_vec(),
                    max.into_vec(),
                    mat
                );

                Rc::new(hitbox) as _
            });

        Self(SharedHit(hitbox_fut))
    }

    #[pyclassmethod]
    fn bvh_node(_cls: PyClassRef, objects: PyListRef, vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let world_futures: Vec<_> = objects
            .borrow_elements()
            .iter()
            .map(|py_obj| {
                let shape = <PyRef<PyShape>>::try_from_object(vm, py_obj.clone())?;
                Ok(shape.shared_hit().get().shared())
            })
            .collect::<PyResult<_>>()?;

        let node_future = future::join_all(world_futures)
            .map(|mut world| {
                let node = BVHNode::new(&mut world, 0., 1.);
                Rc::new(node) as _
            });

        Ok(Self(SharedHit(PyFuture::new(node_future))))
    }

    #[pymethod]
    fn flip_normals(&self) -> Self {
        Self(self.shared_hit().map(|h| h.flip_normals()))
    }

    #[pymethod]
    fn rotate_x(&self, angle: FloatLike) -> Self {
        Self(self.shared_hit().map(move |h| h.rotate_x(angle.as_f32())))
    }

    #[pymethod]
    fn rotate_y(&self, angle: FloatLike) -> Self {
        Self(self.shared_hit().map(move |h| h.rotate_y(angle.as_f32())))
    }

    #[pymethod]
    fn rotate_z(&self, angle: FloatLike) -> Self {
        Self(self.shared_hit().map(move |h| h.rotate_z(angle.as_f32())))
    }

    #[pymethod]
    fn translate(&self, offset: PyVec3) -> Self {
        Self(self.shared_hit().map(move |h| h.translate(offset.into_vec())))
    }

    #[pymethod]
    fn constant_medium(&self, density: FloatLike, color: PyVec3) -> Self {
        Self(self.shared_hit().map(move |h| h.constant_medium(density.as_f32(), color.into_vec())))
    }
}
