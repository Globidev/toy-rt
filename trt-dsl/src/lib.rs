mod prelude;
mod bindings;
mod future;

use prelude::*;

use bindings::SceneInjector;

use rustpython_compiler::{compile::Mode as CompileMode, error::CompileError};
use rpy::{PySettings, InitParameter};

pub use rpy::VirtualMachine;
pub use bindings::{DynScene, DynSceneResult};

pub fn eval_scene(vm: &VirtualMachine, source: &str) -> Result<Option<impl Future<Output = DynSceneResult>>, EvalError> {
    let scene_injector = SceneInjector::new(vm)?;

    let scope = vm.new_scope_with_builtins();
    let code = vm.compile(source, CompileMode::Exec, String::from("User script"))?;
    vm.run_code_obj(code, scope)?;

    Ok(scene_injector.retrieve()?.map(|fut| fut.shared()))
}

pub fn new_vm() -> VirtualMachine {
    let mut settings = PySettings::default();
    settings.initialization_parameter = InitParameter::NoInitialize;

    let mut vm = VirtualMachine::new(settings);
    bindings::init_module(&vm);
    vm.initialize(InitParameter::InitializeInternal);

    vm
}

#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("Failed to compile: {0}")]
    Compile(#[from] CompileError),
    #[error("Python exception: {0:?}")]
    Exception(PyBaseExceptionRef),
}

impl From<PyBaseExceptionRef> for EvalError {
    fn from(ex: PyBaseExceptionRef) -> Self { Self::Exception(ex) }
}

impl EvalError {
    pub fn pretty_print(&self, vm: &VirtualMachine) -> String {
        match self {
            EvalError::Compile(c) => c.to_string(),
            EvalError::Exception(e) => {
                let mut s = Vec::new();
                rpy::exceptions::write_exception(&mut s, vm, e).unwrap();
                String::from_utf8(s).unwrap_or_else(|_| format!("{:?}", e))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_valid_scenes {
        ($($name:ident,)*) => {
            $(
                #[test]
                fn $name() {
                    let vm = new_vm();
                    let scene_code = include_str!(concat!("../scenes/", stringify!($name), ".py"));
                    let scene_res = eval_scene(&vm, scene_code);
                    match scene_res {
                        Ok(opt_scene) => assert!(opt_scene.is_some()),
                        Err(e) => panic!("{}", e.pretty_print(&vm))
                    }
                }
            )*
        };
    }

    test_valid_scenes! {
        simple_3_spheres,
        cornell_box,
        foam_cubes,
        sphere_cluster,
    }

    #[test]
    fn dynamic_scene() {
        let vm = new_vm();
        let source =
            std::fs::read_to_string("/home/globi/dev/toy-ray-tracer/trt-dsl/scenes/dynamic.py")
                .expect("Failed to open dynamic scene");
        let res = eval_scene(&vm, &source);
        if let Err(e) = res {
            panic!("{}", e.pretty_print(&vm))
        }
    }
}
