use trt_core::camera::CameraBuilder;

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
};

use super::vec3::PyVec3;

trt_py_class! { "Camera", PyCamera,
    #[derive(Clone)]
    pub struct PyCamera(CameraBuilder);
}

impl PyCamera {
    pub fn builder(self) -> CameraBuilder {
        self.0
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
    fn tp_new(_cls: PyClassRef, args: PyCameraArgs, _vm: &rpy::VirtualMachine) -> Self {
        let builder = CameraBuilder::default()
            .look_from(args.look_from.into_vec())
            .look_at(args.look_at.into_vec());

        Self(builder)
    }
}
