use trt_core::{hit::RectBuilder, prelude::*};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use std::fmt;
use std::cell::RefCell;
use rpy::{pyobject::PyObjectRef, obj::objlist::PyList};
use super::sphere::PySphere;

#[rpy::pyclass(name = "Scene")]
pub struct PyScene(RefCell<Vec<Box<dyn Hit>>>);

impl fmt::Debug for PyScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Scene of {} objects>", self.0.borrow().len())
    }
}

impl PyScene {
    pub fn take(&self) -> Vec<Box<dyn Hit>> {
        std::mem::take(self.0.borrow_mut().as_mut())
    }
}

impl PyValue for PyScene {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Scene")
    }
}

#[derive(Debug, rpy::FromArgs)]
struct PySceneArgs {
    world: PyObjectRef
}

#[rpy::pyimpl]
impl PyScene {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySceneArgs, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let as_py_list = args.world.downcast::<PyList>()?;

        let mut world: Vec<_> = as_py_list.elements
            .borrow()
            .iter()
            .map(|py_obj| {
                let as_sphere = py_obj.clone().downcast::<PySphere>()?;
                Ok(Box::new((*as_sphere).clone().into_hit()) as Box<dyn Hit>)
            })
            .collect::<PyResult<_>>()?;

        world.push(Box::new(
            RectBuilder
                .x(-50..=50)
                .z(-50..=50)
                .y(50)
                .diffuse_color((7, 7, 7)),
        ));

        Ok(Self(RefCell::new(world)))
    }
}
