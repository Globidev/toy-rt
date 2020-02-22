pub mod camera;
pub mod sphere;
pub mod vec3;
pub mod float;
pub mod scene;
pub mod rect;
pub mod hitbox;
pub mod bvh;
pub mod material;
pub mod shape;

const TRT_MODULE_NAME: &str = "trt";

use rand::prelude::{Rng, StdRng, SeedableRng};
use core::cell::RefCell;
use rustpython_vm::{
    self as rpy,
    pyobject::{PyObjectRef, PyClassImpl, PyResult, TryIntoRef},
    obj::objtuple::PyTupleRef,
    VirtualMachine
};
use std::{fmt, rc::Rc};
use trt_core::hit::Hit;
use crate::future::PyFuture;

#[derive(Clone)]
pub struct SharedHit(PyFuture<Rc<dyn Hit>>);

impl fmt::Debug for SharedHit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<SharedHit object>")
    }
}

impl SharedHit {
    pub fn new<T: Hit + 'static>(hit: T) -> Self {
        Self(PyFuture::ready(Rc::new(hit)))
    }

    pub fn get(&self) -> &PyFuture<Rc<dyn Hit>> {
        &self.0
    }

    pub fn map<T: Hit + 'static>(self, f: impl FnOnce(Rc<dyn Hit>) -> T + 'static) -> Self {
        Self(self.0.map(|x| Rc::new(f(x)) as _))
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

thread_local! {
    static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
}

fn make_trt_module(vm: &VirtualMachine) -> PyObjectRef {
    rpy::py_module!(vm, TRT_MODULE_NAME, {
        "Sphere" => sphere::PySphere::make_class(&vm.ctx),
        "Scene" => scene::PyScene::make_class(&vm.ctx),
        "Camera" => camera::PyCamera::make_class(&vm.ctx),
        "Rect" => rect::PyRect::make_class(&vm.ctx),
        "BVHNode" => bvh::PyBVHNode::make_class(&vm.ctx),
        "HitBox" => hitbox::PyHitBox::make_class(&vm.ctx),
        "Material" => material::PyMaterial::make_class(&vm.ctx),
        "Shape" => shape::PyShape::make_class(&vm.ctx),
        "rand" => vm.ctx.new_function(|| {
            RNG.with(|rng_w| {
                let mut guard = rng_w.borrow_mut();
                let rng = guard.get_or_insert_with(|| StdRng::seed_from_u64(0xDEAD_BEEF));
                rng.gen::<f32>()
            })
        }),
    })
}
