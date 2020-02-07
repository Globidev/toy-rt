use trt_core::{
    hit::{Sphere, SphereBuilder},
    prelude::*,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use super::{float::FloatLike, vec3::PyVec3};

#[rpy::pyclass(name = "Sphere")]
#[derive(Debug, Clone)]
pub struct PySphere(SphereBuilder, Vec3);

impl PySphere {
    pub fn into_hit(self) -> impl Hit {
        self.0.matte(self.1)
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
    color: PyVec3,
}

#[rpy::pyimpl]
impl PySphere {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySphereArgs, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let builder = Sphere::builder()
            .radius(args.radius.as_f32())
            .center(args.center.into_vec());

        Ok(Self(builder, args.color.into_vec()))
    }
}
