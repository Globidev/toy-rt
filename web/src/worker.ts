import { expose as comlinkExpose } from "comlink";

import { PythonVM, Scene, setup_panic_hook } from "trt";
import initWasm from "../../trt-wasm/pkg/trt_wasm.js";

export type SceneSize = { width: number; height: number };

export type WorkerState =
  | { kind: "created" }
  | { kind: "loading" }
  | { kind: "loaded" }
  | { kind: "idle" }
  | { kind: "eval" }
  | { kind: "compute"; row: number };

type OnStateChangeCb = (state: WorkerState) => void;

export type EvalResult =
  | { kind: "error"; error: string }
  | { kind: "success"; returnValue: string; sceneDimensions: SceneSize | null };

export class WasmWorker {
  vm: PythonVM | null = null;
  scene: Scene | null = null;
  state: WorkerState = { kind: "created" };
  debounceIdleTimeoutId = 0;

  onStateChange: OnStateChangeCb | null = null;

  async init(wasmData: ArrayBuffer) {
    this.changeState({ kind: "loading" });

    await initWasm(wasmData);
    setup_panic_hook();
    this.vm = PythonVM.new();

    this.changeState({ kind: "loaded" });
  }

  changeState(state: WorkerState) {
    this.state = state;
    if (state.kind == "idle") {
      this.debounceIdleTimeoutId = self.setTimeout(
        () => this.notifyChangeState(state),
        100
      );
    } else {
      clearTimeout(this.debounceIdleTimeoutId);
      this.notifyChangeState(state);
    }
  }

  notifyChangeState(state: WorkerState) {
    if (this.onStateChange != null) this.onStateChange(state);
  }

  setOnStateChange(cb: OnStateChangeCb) {
    this.onStateChange = cb;
  }

  compute(row: number) {
    if (this.scene != null) {
      this.changeState({ kind: "compute", row: this.scene.height() - row });
      let colors = this.scene.row_color(row);
      this.changeState({ kind: "idle" });
      return colors;
    }
    return null;
  }

  async eval(source: string): Promise<EvalResult> {
    if (this.vm === null)
      return { kind: "success", returnValue: "", sceneDimensions: null };

    this.changeState({ kind: "eval" });
    try {
      let result = this.vm.eval(source);

      let evalResult: EvalResult = {
        kind: "success",
        returnValue: result.data(),
        sceneDimensions: null,
      };

      let scene: Scene | undefined = await result.build_scene();
      if (scene !== undefined) {
        this.scene = scene;
        evalResult.sceneDimensions = {
          width: scene.width(),
          height: scene.height(),
        };
      }

      return evalResult;
    } catch (error) {
      return { kind: "error", error };
    } finally {
      this.changeState({ kind: "idle" });
    }
  }

  localNames(): string[] {
    if (this.vm === null) return [];

    try {
      let result = this.vm
        .eval(`'[{}]'.format(",".join(f'"{k}"' for k in locals().keys()))`)
        .data();
      return JSON.parse(result);
    } catch (error) {
      return [];
    }
  }
}

comlinkExpose(new WasmWorker());
