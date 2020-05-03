#![feature(type_alias_impl_trait)]

use std::{rc::Rc, future::Future};

use wasm_bindgen::prelude::*;

use trt_core::prelude::*;
use trt_dsl::{DynScene, DynSceneResult};
use rand::{SeedableRng, prelude::SmallRng};

#[wasm_bindgen]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once()
}

#[wasm_bindgen]
pub struct PythonVM(trt_dsl::VirtualMachine);

#[wasm_bindgen]
impl PythonVM {
    pub fn new() -> PythonVM {
        Self(trt_dsl::new_vm())
    }

    fn eval_impl(&self, source: &str) -> Result<Option<SceneFuture>, JsValue> {
        trt_dsl::eval_scene(&self.0, &source)
            .map_err(|e| format!("Failed to eval scene: {}", e.pretty_print(&self.0)).into())
    }

    pub fn eval(&self, source: &str) -> Result<Option<ScenePromise>, JsValue> {
        let scene_opt = self.eval_impl(source)?;

        Ok(scene_opt.map(ScenePromise))
    }
}

#[wasm_bindgen]
pub struct ScenePromise(SceneFuture);

#[wasm_bindgen]
impl ScenePromise {
    pub async fn build_scene(self) -> Result<Scene, JsValue> {
        let dyn_scene = self.0.await
            .map_err(|e| format!("{:?}", e))?;

        Ok(Scene(dyn_scene, SmallRng::from_entropy()))
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
