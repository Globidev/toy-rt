use trt_core::prelude::Vec3;

use rustpython_vm as rpy;
use rpy::{
    pyobject::{PyObjectRef, PyResult, TryFromObject},
    obj::objtuple::PyTuple,
    VirtualMachine,
};

use super::float::FloatLike;

#[rpy::pyclass(name = "Vec3")]
#[derive(Debug, Clone, Copy)]
pub struct PyVec3(Vec3);

impl PyVec3 {
    pub fn into_vec(self) -> Vec3 {
        self.0
    }
}

impl TryFromObject for PyVec3 {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        let as_py_tuple = obj.downcast::<PyTuple>()?;

        let vec3 = Vec3::new(
            FloatLike::try_from_object(vm, as_py_tuple.fast_getitem(0))?.as_f32(),
            FloatLike::try_from_object(vm, as_py_tuple.fast_getitem(1))?.as_f32(),
            FloatLike::try_from_object(vm, as_py_tuple.fast_getitem(2))?.as_f32(),
        );

        Ok(Self(vec3))
    }
}
