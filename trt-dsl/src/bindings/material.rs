use trt_core::{
    prelude::*, material::{Dielectric, Lambertian, Diffuse, Metal},
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use std::{fmt, rc::Rc};

use super::{float::FloatLike, vec3::PyVec3};
use rpy::pyobject::{TryFromObject, PyObjectRef, PyRef};
type SharedMaterial = Rc<dyn Material>;

#[rpy::pyclass(name = "Material")]
// #[derive(Debug)]
pub struct PyMaterial(SharedMaterial);

impl fmt::Debug for PyMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Material>")
    }
}

impl PyMaterial {
    pub fn shared_material(&self) -> SharedMaterial {
        self.0.clone()
    }
}

impl PyValue for PyMaterial {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Material")
    }
}

impl TryFromObject for PyMaterial {
    fn try_from_object(vm: &rpy::VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        let x: PyRef<Self> = obj.downcast()
            .map_err(|e| vm.new_type_error(format!("Expected Material, got: {}", e)))?;
        Ok(Self(x.shared_material().clone()))
    }
}

#[rpy::pyimpl]
impl PyMaterial {
    #[pyclassmethod]
    fn dielectric(_cls: PyClassRef, ref_idx: FloatLike, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        Ok(Self(Rc::new(Dielectric::new(ref_idx.as_f32()))))
    }

    #[pyclassmethod]
    fn diffuse_color(_cls: PyClassRef, color: PyVec3, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        Ok(Self(Rc::new(Diffuse::colored(color.into_vec()))))
    }

    #[pyclassmethod]
    fn matte(_cls: PyClassRef, color: PyVec3, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        Ok(Self(Rc::new(Lambertian::colored(color.into_vec()))))
    }

    #[pyclassmethod]
    fn metallic(_cls: PyClassRef, albedo: PyVec3, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        Ok(Self(Rc::new(Metal::new(albedo.into_vec(), 0.))))
    }

    #[pyclassmethod]
    fn metallic_fuzzed(_cls: PyClassRef, albedo: PyVec3, fuzz: FloatLike, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        Ok(Self(Rc::new(Metal::new(albedo.into_vec(), fuzz.as_f32()))))
    }
}
