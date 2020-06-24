#![feature(type_alias_impl_trait)]

use std::{rc::Rc, future::Future};

use wasm_bindgen::prelude::*;

use trt_core::prelude::*;
use trt_dsl::{DynScene, DynSceneResult, EvalOutput};
use rand::{SeedableRng, prelude::SmallRng};

#[wasm_bindgen]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once()
}

#[wasm_bindgen]
pub struct PythonVM(trt_dsl::VirtualMachine, trt_dsl::Scope);

#[wasm_bindgen]
impl PythonVM {
    pub fn new() -> PythonVM {
        let vm = trt_dsl::new_vm();
        let scope = vm.new_scope_with_builtins();
        Self(vm, scope)
    }

    fn eval_impl(&self, source: &str) -> Result<EvalOutput<SceneFuture>, JsValue> {
        trt_dsl::eval(&self.0, source, self.1.clone())
            .map_err(|e| e.pretty_print(&self.0).into())
    }

    pub fn eval(&self, source: &str) -> Result<WasmEvalOutput, JsValue> {
        let EvalOutput { data, rendered_scene } = self.eval_impl(source)?;

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
