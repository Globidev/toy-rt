#![feature(type_alias_impl_trait)]

use std::{rc::Rc, future::Future, io::Write};

use wasm_bindgen::{JsCast, prelude::*};

use trt_core::prelude::*;
use trt_dsl::{DynScene, DynSceneResult, EvalOutput, CompileMode};
use rand::{SeedableRng, prelude::SmallRng};

#[wasm_bindgen]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once()
}

struct FnWriter(js_sys::Function);
unsafe impl Send for FnWriter { }

impl Write for FnWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.call1(&JsValue::UNDEFINED, &std::str::from_utf8(buf).expect("Not utf8").into())
            .expect("Failed to write");
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.call1(&JsValue::UNDEFINED, &"[FLUSH]".into())
            .expect("Failed to write");
        Ok(())
    }
}

#[wasm_bindgen]
pub struct PythonVM(trt_dsl::VirtualMachine, trt_dsl::Scope);

#[wasm_bindgen]
pub enum EvalMode {
    Verbose,
    Silent,
}

#[wasm_bindgen]
impl PythonVM {
    pub fn new(write_callback: JsValue) -> PythonVM {
        let write_callback_fn = write_callback
            .dyn_into()
            .expect("Not a function");

        let foo = FnWriter(write_callback_fn);
        let vm = trt_dsl::new_vm(foo);
        let scope = vm.new_scope_with_builtins();
        Self(vm, scope)
    }

    fn eval_impl(&self, source: &str, mode: EvalMode) -> Result<EvalOutput<SceneFuture>, JsValue> {
        let compile_mode = match mode {
            EvalMode::Verbose => CompileMode::Single,
            EvalMode::Silent => CompileMode::Eval,
        };
        trt_dsl::eval(&self.0, source, self.1.clone(), compile_mode)
            .map_err(|e| e.pretty_print(&self.0).into())
    }

    pub fn eval(&self, source: &str, mode: EvalMode) -> Result<WasmEvalOutput, JsValue> {
        let EvalOutput { data, rendered_scene } = self.eval_impl(source, mode)?;

        Ok(WasmEvalOutput {
            data,
            rendered_scene
        })
    }
}

#[wasm_bindgen(js_name=EvalResult)]
pub struct WasmEvalOutput {
    data: String,
    rendered_scene: Option<SceneFuture>
}

#[wasm_bindgen(js_class=EvalResult)]
impl WasmEvalOutput {
    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub async fn build_scene(self) -> Result<Option<Scene>, JsValue> {
        match self.rendered_scene {
            Some(scene_fut) => {
                let dyn_scene = scene_fut.await
                    .map_err(|e| format!("{}", e))?;

                Ok(Some(Scene(dyn_scene, SmallRng::from_entropy())))
            },
            None => Ok(None)
        }
    }
}

type SceneFuture = impl Future<Output = DynSceneResult>;

#[wasm_bindgen]
pub struct Scene(Rc<DynScene>, SmallRng);

#[wasm_bindgen]
impl Scene {
    pub fn row_color(&mut self, y: usize) -> Vec<u32> {
        (0..self.0.width)
            .map(|x| self.pixel_color(x, y))
            .collect()
    }

    pub fn pixel_color(&mut self, x: usize, y: usize) -> u32 {
        let Color(r, g, b) = self.0.pixel_color((x, y), &mut self.1);
        u32::from_be_bytes([0, r, g, b])
    }

    pub fn width(&self) -> u32 {
        self.0.width as _
    }

    pub fn height(&self) -> u32 {
        self.0.height as _
    }
}
