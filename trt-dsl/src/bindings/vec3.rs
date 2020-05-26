use crate::prelude::*;
use super::float::FloatLike;

use rpy::obj::objtuple::PyTupleRef;

use trt_core::prelude::Vec3;

trt_py_class! { "Vec3", PyVec3,
    #[derive(Clone, Copy)]
    pub struct PyVec3(Vec3);
}

impl PyVec3 {
    pub fn into_vec(self) -> Vec3 {
        self.0
    }
}

impl TryFromObject for PyVec3 {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        let as_py_tuple: PyTupleRef = obj.try_into_ref(vm)?;
        let as_slice = as_py_tuple.as_slice();

        if as_slice.len() != 3 {
            return Err(vm.new_type_error("Expected a tuple of 3 elements".to_owned()))
        }

        let vec3 = Vec3::new(
            FloatLike::try_from_object(vm, as_slice[0].clone())?.as_f32(),
            FloatLike::try_from_object(vm, as_slice[1].clone())?.as_f32(),
            FloatLike::try_from_object(vm, as_slice[2].clone())?.as_f32(),
        );

        Ok(Self(vec3))
    }
}
