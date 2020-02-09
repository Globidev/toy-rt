pub mod camera;
pub mod sphere;
pub mod vec3;
pub mod float;
pub mod scene;
pub mod rect;
pub mod material;

const TRT_MODULE_NAME: &str = "trt";

use rustpython_vm::{
    self as rpy,
    pyobject::{PyObjectRef, PyClassImpl, PyResult, TryIntoRef},
    obj::objtuple::PyTupleRef,
    VirtualMachine
};
use std::{fmt, rc::Rc, ops::Deref};
use trt_core::hit::Hit;

#[derive(Clone)]
pub struct SharedHit(Rc<dyn Hit>);

impl fmt::Debug for SharedHit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<SharedHit object>")
    }
}

impl Deref for SharedHit {
    type Target = Rc<dyn Hit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SharedHit {
    pub fn new<T: Hit + 'static>(hit: T) -> Self {
        Self(Rc::new(hit))
    }
}

pub fn init_module(vm: &VirtualMachine) -> PyResult<()> {
    rpy::import::init_importlib(&vm, rpy::InitParameter::InitializeInternal)?;

    vm.stdlib_inits
        .borrow_mut()
        .insert(TRT_MODULE_NAME.to_owned(), Box::new(make_trt_module));

    let builtin_names: PyTupleRef = vm.get_attribute(vm.sys_module.clone(), "builtin_module_names")?
        .try_into_ref(&vm)?;

    let mut new_builtins = builtin_names.as_slice().to_owned();
    new_builtins.push(vm.new_str(TRT_MODULE_NAME.to_owned()));

    vm.set_attr(&vm.sys_module, "builtin_module_names", vm.ctx.new_tuple(new_builtins))?;

    Ok(())
}

fn make_trt_module(vm: &VirtualMachine) -> PyObjectRef {
    rpy::py_module!(vm, TRT_MODULE_NAME, {
        "Sphere" => sphere::PySphere::make_class(&vm.ctx),
        "Scene" => scene::PyScene::make_class(&vm.ctx),
        "Camera" => camera::PyCamera::make_class(&vm.ctx),
        "Rect" => rect::PyRect::make_class(&vm.ctx),
        "Material" => material::PyMaterial::make_class(&vm.ctx),
    })
}
