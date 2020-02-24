mod bindings;
mod future;

pub use crate::bindings::material::MaterialError;
use rustpython_vm::pyobject::ItemProtocol;
use rustpython_vm::{self as rpy, exceptions::PyBaseExceptionRef};

pub use bindings::scene::{PyScene, DynScene};
use rustpython_compiler::{compile::Mode as CompileMode, error::CompileError};
use rpy::{PySettings, pyobject::{TryIntoRef, PyRef, PyValue}, InitParameter};
pub use rpy::{obj::{objdict::PyDictRef, objnone::PyNone}, VirtualMachine, scope::Scope};
use std::{rc::Rc, future::Future};

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

pub fn eval_scene(vm: &rpy::VirtualMachine, source: &str) -> Result<impl Future<Output = Result<Rc<DynScene>, Rc<MaterialError>>>, EvalError> {
    let scope = vm.new_scope_with_builtins();
    let module = vm.import("_trt", &[], 0)?;
    module.dict.as_ref().unwrap().borrow().set_item("__render_scene", PyNone.into_ref(vm).into(), vm)?;

    let code = vm.compile(source, CompileMode::Exec, "test".to_string())?;
    vm.run_code_obj(code, scope.clone())?;

    let result = module.dict.as_ref().unwrap().borrow().get_item_option("__render_scene", vm)?;
    dbg!(&result);

    let py_scene: PyRef<PyScene> = result.unwrap().try_into_ref(vm)?;

    Ok(py_scene.get().shared())
}

pub fn new_vm() -> rpy::VirtualMachine {
    let mut settings = PySettings::default();
    settings.initialization_parameter = InitParameter::NoInitialize;

    let mut vm = rpy::VirtualMachine::new(settings);

    bindings::init_module(&vm);

    vm.initialize(InitParameter::InitializeInternal);

    vm
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_scene() {
        let vm = new_vm();
        let res = eval_scene(&vm, include_str!("../scenes/demo.py"));
        if let Err(e) = res {
            panic!("{}", e.pretty_print(&vm))
        }
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

        let res = eval_scene(&vm, "");
        if let Err(e) = res {
            panic!("{}", e.pretty_print(&vm))
        }

        panic!()
    }
}
