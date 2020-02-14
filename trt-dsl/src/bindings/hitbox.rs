use trt_core::{
    hit::HitBox,
    prelude::*,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use super::{SharedHit, material::PyMaterial, vec3::PyVec3, float::FloatLike};

#[rpy::pyclass(name = "HitBox")]
#[derive(Debug)]
pub struct PyHitBox(SharedHit);

impl PyHitBox {
    pub fn shared_hit(&self) -> SharedHit {
        self.0.clone()
    }
}

impl PyValue for PyHitBox {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "HitBox")
    }
}

#[derive(Debug, rpy::FromArgs)]
struct PyHitBoxArgs {
    min: PyVec3,
    max: PyVec3,
    material: PyMaterial,
}

#[rpy::pyimpl]
impl PyHitBox {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PyHitBoxArgs, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let material = args.material.shared_material();

        let hitbox = HitBox::new(
            args.min.into_vec(),
            args.max.into_vec(),
            material
        );

        Ok(Self(SharedHit::new(hitbox)))
    }

    #[pymethod]
    fn flip_normals(&self) -> Self {
        Self(SharedHit::new((*self.shared_hit()).clone().flip_normals()))
    }

    #[pymethod]
    fn rotate_y(&self, angle: FloatLike) -> Self {
        Self(SharedHit::new((*self.shared_hit()).clone().rotate_y(angle.as_f32())))
    }

    #[pymethod]
    fn translate(&self, offset: PyVec3) -> Self {
        Self(SharedHit::new((*self.shared_hit()).clone().translate(offset.into_vec())))
    }

    #[pymethod]
    fn constant_medium(&self, density: FloatLike, color: PyVec3) -> Self {
        Self(SharedHit::new((*self.shared_hit()).clone().constant_medium(density.as_f32(), color.into_vec())))
    }
}
