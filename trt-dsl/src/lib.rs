mod prelude;
mod bindings;
mod future;

use prelude::*;

use bindings::SceneInjector;

use rustpython_compiler::{compile::Mode as CompileMode, error::CompileError};
use rpy::{PySettings, InitParameter};

pub use rpy::{VirtualMachine, scope::Scope, obj::objstr::PyStringRef};
pub use bindings::{DynScene, DynSceneResult};
use std::{sync::Mutex, io::Write};

pub struct EvalOutput<F> {
    pub rendered_scene: Option<F>,
    pub data: String,
}

pub fn eval(vm: &VirtualMachine, source: &str, scope: Scope) -> Result<EvalOutput<impl Future<Output = DynSceneResult>>, EvalError> {
    let scene_injector = SceneInjector::new(vm)?;

    let code = vm.compile(source, CompileMode::Single, String::from("<webconsole>"))?;
    let result = vm.run_code_obj(code, scope)?;

    let result_data = vm.to_str(&result)?.to_string();
    let scene = scene_injector.retrieve()?.map(|fut| fut.shared());

    Ok(EvalOutput { rendered_scene: scene, data: result_data })
}

pub fn new_vm(writer: impl Write + Send + 'static) -> VirtualMachine {
    let mut settings = PySettings::default();
    settings.initialization_parameter = InitParameter::NoInitialize;

    let mut vm = VirtualMachine::new(settings);
    bindings::init_module(&vm);
    vm.initialize(InitParameter::InitializeInternal);

    let ctx = &vm.ctx;

    let thread_safe_writer = Mutex::new(writer);
    let write = ctx.new_method(move |_self: PyObjectRef, data: PyStringRef, vm: &VirtualMachine| {
        thread_safe_writer.lock().unwrap().write(data.as_str().as_bytes())
            .unwrap();
    });
    let flush = ctx.new_method(|| ());

    let stdout = ctx.new_base_object(rpy::py_class!(ctx, "WasmStdout", ctx.object(), {
        "write" => write,
        "flush" => flush
    }), None);

    vm.set_attr(&vm.sys_module, "stdout", stdout).unwrap();

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
                    let vm = new_vm(Vec::new());
                    let scene_code = include_str!(concat!("../scenes/", stringify!($name), ".py"));
                    let eval_res = eval(&vm, scene_code, vm.new_scope_with_builtins());
                    match eval_res {
                        Ok(ret) => assert!(ret.rendered_scene.is_some()),
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
        let vm = new_vm(Vec::new());
        let source =
            std::fs::read_to_string("/home/globi/dev/toy-ray-tracer/trt-dsl/scenes/dynamic.py")
                .expect("Failed to open dynamic scene");
        let res = eval(&vm, &source, vm.new_scope_with_builtins());
        if let Err(e) = res {
            panic!("{}", e.pretty_print(&vm))
        }
    }
}
