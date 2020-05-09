use crate::{future::PyFuture, prelude::*};

use rpy::py_compile_bytecode;

const TRT_INTERNAL_MODULE_NAME: &str = "_trt";

macro_rules! trt_py_class {
    ($py_name:literal, $name:ident, $item:item) => {
        #[rpy::pyclass(name = $py_name)]
        $item

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, concat!("<", $py_name, " object>"))
            }
        }

        impl ::rustpython_vm::pyobject::PyValue for $name {
            fn class(vm: &::rustpython_vm::VirtualMachine) -> ::rustpython_vm::obj::objtype::PyClassRef {
                vm.class(crate::bindings::TRT_INTERNAL_MODULE_NAME, $py_name)
            }
        }
    };
}

mod camera;
mod vec3;
mod float;
mod scene;
mod material;
mod shape;

pub use scene::{DynScene, DynSceneResult};

pub fn init_module(vm: &VirtualMachine) {
    vm.stdlib_inits
        .borrow_mut()
        .insert(TRT_INTERNAL_MODULE_NAME.to_owned(), Box::new(make_trt_module));

    vm.frozen.borrow_mut()
        .extend(py_compile_bytecode!(
            dir = "src/api",
            module_name = "trt",
        ));
}

const RENDER_SCENE_IDENT: &str = "__render_scene";

pub struct SceneInjector<'vm> {
    trt_module_dict: PyDictRef,
    vm: &'vm VirtualMachine,
}

impl<'vm> SceneInjector<'vm> {
    pub fn new(vm: &'vm VirtualMachine) -> Result<Self, PyBaseExceptionRef> {
        let module = vm.import(TRT_INTERNAL_MODULE_NAME, &[], 0)?;

        let module_dict = module.dict()
            .expect("Module should have a dict");

        module_dict
            .set_item(RENDER_SCENE_IDENT, vm.ctx.none(), vm)?;

        Ok(Self {
            trt_module_dict: module_dict,
            vm,
        })
    }

    pub fn retrieve(&self) -> Result<Option<PyFuture<DynSceneResult>>, PyBaseExceptionRef> {
        let render_scene = self.trt_module_dict
            .get_item_option(RENDER_SCENE_IDENT, self.vm)?;

        match render_scene {
            None => Ok(None),
            Some(obj) => {
                if self.vm.is_none(&obj) {
                    Ok(None)
                } else {
                    let py_scene: PyRef<scene::PyScene> = obj.try_into_ref(self.vm)?;
                    Ok(Some(py_scene.0.clone()))
                }
            }
        }
    }
}

fn make_trt_module(vm: &VirtualMachine) -> PyObjectRef {
    rpy::py_module!(vm, TRT_INTERNAL_MODULE_NAME, {
        "Material" => material::PyMaterial::make_class(&vm.ctx),
        "Shape" => shape::PyShape::make_class(&vm.ctx),
        "Scene" => scene::PyScene::make_class(&vm.ctx),
        "Camera" => camera::PyCamera::make_class(&vm.ctx),
    })
}
