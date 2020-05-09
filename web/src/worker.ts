import { expose as comlinkExpose } from "comlink";

import { PythonVM, Scene } from "trt";

type SceneSize = { width: number; height: number };

export class WasmWorker {
  vm: PythonVM | null = null;
  scene: Scene | null = null;

  async init() {
    let wasm = await import("trt");

    wasm.setup_panic_hook();
    this.vm = wasm.PythonVM.new();
  }

  async compileScene(code: string): Promise<SceneSize | null> {
    if (this.vm) {
      const result = this.vm.eval(code);

      let scene: Scene | null = await result.build_scene();

      if (scene) {
        this.scene = scene;
        return { width: scene.width(), height: scene.height() };
      }
    }
    return null;
  }

  compute(row: number) {
    return this.scene?.row_color(row);
  }

  eval(source: string) {
    return this.vm?.eval(source).data();
  }
}

comlinkExpose(new WasmWorker());
