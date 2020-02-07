use trt_core::camera::CameraBuilder;

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use super::vec3::PyVec3;

#[rpy::pyclass(name = "Camera")]
#[derive(Debug, Clone)]
pub struct PyCamera(CameraBuilder);

impl PyCamera {
    pub fn builder(self) -> CameraBuilder {
        self.0
    }
}

impl PyValue for PyCamera {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Camera")
    }
}

#[derive(Debug, rpy::FromArgs)]
struct PyCameraArgs {
    look_from: PyVec3,
    look_at: PyVec3,
}

#[rpy::pyimpl]
impl PyCamera {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PyCameraArgs, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let builder = CameraBuilder::default()
            .look_from(args.look_from.into_vec())
            .look_at(args.look_at.into_vec());

        Ok(Self(builder))
    }
}
