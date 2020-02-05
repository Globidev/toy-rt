use trt_core::prelude::*;

use rustpython_vm as rpy;
use rpy::{
    pyobject::{PyResult, PyObjectRef},
    obj::objdict::PyDict,
    obj::objlist::PyList,
};

use rustpython_compiler::{compile::Mode as CompileMode, error::CompileError};
use bindings::scene::PyScene;

mod bindings;

#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("Error compiling snippet: {0}")]
    Compile(CompileError),
    #[error("Error initializing python module: {0}")]
    InitModule(PyObjectRef),
    #[error("Error executing scene code: {0}")]
    Exec(PyObjectRef),
    #[error("Error evaluating scene code: {0}")]
    Eval(PyObjectRef),
}

impl EvalError {
    pub fn pretty_print(&self, vm: &rpy::VirtualMachine) -> String {
        match self {
            EvalError::Compile(c) => c.to_string(),
            EvalError::InitModule(p)
            | EvalError::Exec(p)
            | EvalError::Eval(p) => {
                vm.to_pystr(p).unwrap()
            },
        }
    }
}

pub fn eval_scene(vm: &rpy::VirtualMachine, source: &str) -> Result<impl Hit, EvalError> {

    let scope = vm.new_scope_with_builtins();

    let code = vm.compile(source, CompileMode::Exec, "test".to_string())
        .map_err(EvalError::Compile)?;

    vm.run_code_obj(code, scope.clone())
        .map_err(EvalError::Exec)?;

    let code = vm.compile("scene()", CompileMode::Eval, "test".to_string())
        .map_err(EvalError::Compile)?;

    let result = vm.run_code_obj(code, scope)
        .map_err(EvalError::Eval)?;

    let scene = result.downcast::<PyScene>()
        .map_err(EvalError::Eval)?;

    let world = scene.take();

    Ok(trt_core::hit::HitList::new(world))
}

pub fn new_vm() -> Result<rpy::VirtualMachine, EvalError> {
    let vm = rpy::VirtualMachine::default();

    bindings::init_module(&vm)
        .map_err(EvalError::InitModule)?;

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
    }
}
