import * as Comlink from "comlink";
import { WasmWorker, SceneSize } from "./worker";

import { createNanoEvents } from "nanoevents";
import { EvalMode } from "trt";

interface ExecutorEvents {
  lineComputed: (
    y: number,
    width: number,
    height: number,
    colors: Uint32Array
  ) => void;
  sceneLoaded: (width: number, height: number) => void;
  sceneRendered: (ms: number) => void;
  evalError: (error: string) => void;
  stdoutWritten: (text: string) => void;
}

export class WasmExecutor {
  workers: Comlink.Remote<WasmWorker>[];
  enabledWorkers: number = 0;
  cancelCurrentRender: boolean = false;

  events = createNanoEvents<ExecutorEvents>();

  constructor(workerCount: number) {
    this.workers = Array(Math.max(1, workerCount))
      .fill(null)
      .map(() => {
        const rawWorker = new Worker("./worker.js");
        return Comlink.wrap(rawWorker);
      });
  }

  async init(wasmData: ArrayBuffer) {
    let initWorkers = this.workers.map((worker, i) => {
      let onStdout;
      if (i == 0) {
        onStdout = (s: string) => this.events.emit("stdoutWritten", s);
      } else {
        onStdout = (_: string) => {};
      }
      return worker.init(wasmData, Comlink.proxy(onStdout));
    });
    await Promise.all(initWorkers);
  }

  async eval(code: string, mode: EvalMode) {
    const results = await Promise.all(
      this.workers.map(async (w) => {
        return await w.eval(code, mode);
      })
    );
    const result = results[0];

    if (result.kind == "error") this.events.emit("evalError", result.error);

    return result;
  }

  async getLocalNames() {
    return await this.workers[0].localNames();
  }

  async render(sceneSize: SceneSize, sceneCode: string) {
    this.cancelCurrentRender = false;

    let { width, height } = sceneSize;

    this.events.emit("sceneLoaded", width, height);

    let startTime = performance.now();
    let rows = height - 1;
    let work = this.workers.map(async (worker, i) => {
      // if (i != 0) await worker.eval(sceneCode, EvalMode.Verbose);

      while (rows >= 0) {
        if (this.cancelCurrentRender) {
          break;
        }

        let row = rows;
        rows -= 1;

        let colors = await worker.compute(row);

        if (colors)
          this.events.emit("lineComputed", row, width, height, colors);
      }
    });

    await Promise.all(work);

    let renderDuration = performance.now() - startTime;

    this.events.emit("sceneRendered", renderDuration);
  }

  cancelRender() {
    this.cancelCurrentRender = true;
  }
}
