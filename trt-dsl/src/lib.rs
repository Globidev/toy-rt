use trt_core::hit::{Sphere, RectBuilder};
use trt_core::prelude::*;

use rustpython_vm as rpy;
use rpy::{
    pyobject::{PyRef, PyValue, PyObjectRef, PyResult, TryIntoRef, PyClassImpl},
    obj::objtype::PyClassRef,
    obj::objtuple::PyTuple,
    obj::objlist::PyList,
    obj::objfloat::PyFloat,
};

use rustpython_compiler::compile::Mode as CompileMode;

pub fn eval_scene(source: &str) -> impl Hit {
    use trt_core::hit::sphere::SphereBuilder;

    #[rpy::pyclass(name = "Sphere")]
    #[derive(Debug)]
    struct PySphere(SphereBuilder, Vec3);

    impl PyValue for PySphere {
        fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
            vm.class("trt", "Sphere")
        }
    }

    #[rpy::pyimpl]
    impl PySphere {
        #[pyslot(new)]
        fn tp_new(
            _cls: PyClassRef,
            args: PySphereArgs,
            _vm: &rpy::VirtualMachine,
        ) -> PyResult<Self> {
            let center = (
                args.center.fast_getitem(0).downcast::<PyFloat>()?.to_f64(),
                args.center.fast_getitem(1).downcast::<PyFloat>()?.to_f64(),
                args.center.fast_getitem(2).downcast::<PyFloat>()?.to_f64(),
            );

            let builder = Sphere::builder()
                .radius(args.radius)
                .center(center);

            let color = (
                args.color.fast_getitem(0).downcast::<PyFloat>()?.to_f64(),
                args.color.fast_getitem(1).downcast::<PyFloat>()?.to_f64(),
                args.color.fast_getitem(2).downcast::<PyFloat>()?.to_f64(),
            );

            Ok(Self(builder, color.into()))
        }

        // #[pymethod(name = "__init__")]
        // fn new(z: PyRef<Self>, x: u32, vm: &rpy::VirtualMachine) -> PyResult<Self> {
        //     Ok(S(x))
        // }
    }

    fn make_trt_module(vm: &rpy::VirtualMachine) -> PyObjectRef {
        rpy::py_module!(vm, "trt", {
            "Sphere" => PySphere::make_class(&vm.ctx),
            // "s" => vm.ctx.new_rustfunc(make_s)
        })
    }

    #[derive(Debug, rpy::FromArgs)]
    struct PySphereArgs {
        // center: (f32, f32, f32),
        center: PyRef<PyTuple>,
        radius: f32,
        color: PyRef<PyTuple>,
    }

    let vm = rpy::VirtualMachine::default();
    let scope = vm.new_scope_with_builtins();

    vm.stdlib_inits
        .borrow_mut()
        .insert("trt".to_string(), Box::new(make_trt_module));

    rpy::import::init_importlib(&vm, false).unwrap();

    let builtin_names: PyRef<PyTuple> = vm.get_attribute(vm.sys_module.clone(), "builtin_module_names")
        .map_err(|err| vm.to_str(&err).unwrap())
        .expect("wat")
        .try_into_ref(&vm)
        .expect("???");

    let mut new_builtins = builtin_names.elements.clone();
    new_builtins.push(vm.new_str("trt".to_owned()));

    vm.set_attr(&vm.sys_module, "builtin_module_names", vm.ctx.new_tuple(new_builtins))
        .expect("lolol");

    let code = vm.compile(source, CompileMode::Exec, "test".to_string())
        .expect("Failed to compile");

    vm.run_code_obj(code, scope.clone())
        .map_err(|err| vm.to_str(&err).unwrap())
        .expect("Failed to run");

    let code = vm.compile("world()", CompileMode::Eval, "test".to_string())
        .expect("Failed to compile");

    let result = vm.run_code_obj(code, scope)
        .map_err(|err| vm.to_str(&err).unwrap())
        .expect("Failed to eval");

    // dbg!(&result.class());
    // dbg!(vm.to_str(&result).unwrap().to_string());
    let world_list = result.downcast::<PyList>().unwrap();

    let mut world: Vec<_> = world_list.elements.borrow().iter()
        .map(|erased_sphere| {
            let sphere = erased_sphere.clone().downcast::<PySphere>().unwrap();
            Box::new(sphere.0.clone().matte(sphere.1)) as Box<dyn Hit>
        })
        .collect();

    world.push(Box::new(
        RectBuilder
            .x(-50..=50)
            .z(-50..=50)
            .y(50)
            .diffuse_color((7, 7, 7)),
    ));

    trt_core::hit::hitlist::HitList::new(world)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_scene() {
       let _ = eval_scene(include_str!("../scenes/demo.py"));
    }
}
