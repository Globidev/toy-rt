mod bindings;
mod future;

use rustpython_vm::{self as rpy, exceptions::PyBaseExceptionRef};

pub use bindings::scene::{PyScene, DynScene};
use rustpython_compiler::{compile::Mode as CompileMode, error::CompileError};
use rpy::{PySettings, pyobject::{TryIntoRef, PyRef}, InitParameter};
pub use rpy::VirtualMachine;
use std::{rc::Rc, future::Future};

#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("Failed to compile: {0}")]
    Compile(#[from] CompileError),
    #[error("Python exception: {0:?}")]
    Exception(PyBaseExceptionRef),
}

impl EvalError {
    pub fn pretty_print(&self, vm: &rpy::VirtualMachine) -> String {
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

pub fn eval_scene(vm: &rpy::VirtualMachine, source: &str) -> Result<impl Future<Output = Rc<DynScene>>, EvalError> {
    let scope = vm.new_scope_with_builtins();

    let code = vm
        .compile(source, CompileMode::Exec, "test".to_string())
        .map_err(EvalError::Compile)?;

    vm.run_code_obj(code, scope.clone())
        .map_err(EvalError::Exception)?;

    let code = vm
        .compile("scene()", CompileMode::Eval, "test".to_string())
        .map_err(EvalError::Compile)?;

    let result = vm.run_code_obj(code, scope).map_err(EvalError::Exception)?;

    let py_scene: PyRef<PyScene> = result.try_into_ref(vm).map_err(EvalError::Exception)?;

    Ok(py_scene.get().shared())
}

pub fn new_vm() -> Result<rpy::VirtualMachine, EvalError> {
    let mut settings = PySettings::default();
    settings.initialization_parameter = InitParameter::InitializeInternal;

    let vm = rpy::VirtualMachine::new(settings);

    bindings::init_module(&vm).map_err(EvalError::Exception)?;

    Ok(vm)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_scene() {
        let vm = new_vm().expect("Failed to init vm");
        let res = eval_scene(&vm, include_str!("../scenes/demo.py"));
        if let Err(e) = res {
            panic!("{}", e.pretty_print(&vm))
        }
    }

    #[test]
    fn dynamic_scene() {
        let vm = new_vm().expect("Failed to init vm");
        let source =
            std::fs::read_to_string("/home/globi/dev/toy-ray-tracer/trt-dsl/scenes/dynamic.py")
                .expect("Failed to open dynamic scene");
        let res = eval_scene(&vm, &source);
        if let Err(e) = res {
            panic!("{}", e.pretty_print(&vm))
        }
    }
}
