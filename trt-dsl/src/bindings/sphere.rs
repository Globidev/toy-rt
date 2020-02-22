use trt_core::{
    hit::Sphere,
    prelude::*,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use super::{float::FloatLike, vec3::PyVec3, SharedHit, material::PyMaterial};
use futures::prelude::*;
use crate::future::PyFuture;
use std::rc::Rc;

#[rpy::pyclass(name = "Sphere")]
#[derive(Debug)]
pub struct PySphere(SharedHit);

impl PySphere {
    pub fn shared_hit(&self) -> SharedHit {
        self.0.clone()
    }
}

impl PyValue for PySphere {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Sphere")
    }
}

#[derive(Debug, rpy::FromArgs)]
struct PySphereArgs {
    center: PyVec3,
    radius: FloatLike,
    material: PyMaterial,
}

#[rpy::pyimpl]
impl PySphere {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySphereArgs, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let sphere_fut = args.material.shared_material().shared()
            .map(move |mat| {
                let sphere = Sphere::builder()
                    .radius(args.radius.as_f32())
                    .center(args.center.into_vec())
                    .material(mat);

                Rc::new(sphere) as _
            });

        Ok(Self(SharedHit(PyFuture::new(sphere_fut))))
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
