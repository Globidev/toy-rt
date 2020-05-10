import * as Comlink from "comlink";
import { WasmWorker, SceneSize } from "./worker";

type OnLineComputedCb = (
  y: number,
  width: number,
  height: number,
  colors: Uint32Array
) => void;
type OnSceneLoadedCb = (width: number, height: number) => void;
type OnSceneRenderedCb = () => void;
type OnEvalErrorCb = (error: string) => void;

export class WasmExecutor {
  workers: Comlink.Remote<WasmWorker>[];
  enabledWorkers: number = 0;
  cancelCurrentRender: boolean = false;

  onLineComputed: OnLineComputedCb = () => {};
  onSceneLoaded: OnSceneLoadedCb = () => {};
  onSceneRendered: OnSceneRenderedCb = () => {};
  onEvalError: OnEvalErrorCb = () => {};

  constructor(workerCount: number) {
    this.workers = Array(Math.max(1, workerCount))
      .fill(null)
      .map(() => {
        const rawWorker = new Worker("./worker.js");
        return Comlink.wrap(rawWorker);
      });
  }

  async init() {
    let initWorkers = this.workers.map((worker) => worker.init());
    await Promise.all(initWorkers);
  }

  async eval(code: string) {
    const result = await this.workers[0].eval(code);

    if (result.kind == "error") this.onEvalError(result.error);

    return result;
  }

  async getLocalNames() {
    return await this.workers[0].localNames();
  }

  async render(sceneSize: SceneSize, sceneCode: string) {
    this.cancelCurrentRender = false;

    let { width, height } = sceneSize;

    this.onSceneLoaded(width, height);

    let rows = height - 1;
    let work = this.workers.map(async (worker, i) => {
      if (i != 0) await worker.eval(sceneCode);

      while (rows >= 0) {
        if (this.cancelCurrentRender) {
          break;
        }

        let row = rows;
        rows -= 1;

        let colors = await worker.compute(row);

        if (colors) this.onLineComputed(row, width, height, colors);
      }
    });

    await Promise.all(work);

    this.onSceneRendered();
  }

  cancelRender() {
    this.cancelCurrentRender = true;
  }
}
