pub mod camera;
pub mod sphere;
pub mod vec3;
pub mod float;
pub mod scene;

const TRT_MODULE_NAME: &str = "trt";

use rustpython_vm::{
    self as rpy,
    pyobject::{PyObjectRef, PyClassImpl, PyResult, TryIntoRef},
    obj::objtuple::PyTupleRef,
    VirtualMachine
};

pub fn init_module(vm: &VirtualMachine) -> PyResult<()> {
    vm.stdlib_inits
        .borrow_mut()
        .insert(TRT_MODULE_NAME.to_owned(), Box::new(make_trt_module));

    rpy::import::init_importlib(&vm, false)?;

    let builtin_names: PyTupleRef = vm.get_attribute(vm.sys_module.clone(), "builtin_module_names")?
        .try_into_ref(&vm)?;

    let mut new_builtins = builtin_names.elements.clone();
    new_builtins.push(vm.new_str(TRT_MODULE_NAME.to_owned()));

    vm.set_attr(&vm.sys_module, "builtin_module_names", vm.ctx.new_tuple(new_builtins))?;

    Ok(())
}

fn make_trt_module(vm: &VirtualMachine) -> PyObjectRef {
    rpy::py_module!(vm, TRT_MODULE_NAME, {
        "Sphere" => sphere::PySphere::make_class(&vm.ctx),
        "Scene" => scene::PyScene::make_class(&vm.ctx),
        "Camera" => camera::PyCamera::make_class(&vm.ctx),
    })
}
