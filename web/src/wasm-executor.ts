import * as Comlink from "comlink";
import { WasmWorker } from "./worker";

type OnLineComputedCb = (
  y: number,
  width: number,
  height: number,
  colors: Uint32Array
) => void;
type OnSceneLoadedCb = (width: number, height: number) => void;
type OnSceneRenderedCb = () => void;

type RemoteWorker = Comlink.Remote<WasmWorker>;

export class WasmExecutor {
  workers: RemoteWorker[];
  enabledWorkers: number = 0;

  onLineComputed: OnLineComputedCb = () => {};
  onSceneLoaded: OnSceneLoadedCb = () => {};
  onSceneRendered: OnSceneRenderedCb = () => {};

  constructor(workers: RemoteWorker[]) {
    this.workers = workers;
  }

  static async new(workerCount: number) {
    let initWorkers = Array(Math.max(1, workerCount))
      .fill(null)
      .map(async () => {
        const rawWorker = new Worker("./worker.js");
        const worker = Comlink.wrap<WasmWorker>(rawWorker);

        await worker.init();

        return worker;
      });

    let workers = await Promise.all(initWorkers);

    return new WasmExecutor(workers);
  }

  async eval(code: string) {
    return await this.workers[0].eval(code);
  }

  async render(sceneCode: string) {
    let sceneSize = await this.workers[0].compileScene(sceneCode);
    if (sceneSize === null) return;

    let { width, height } = sceneSize;
    // let scene: Scene;
    // try {
    //   // @ts-ignore
    //   const scenePromise = this.vm.eval(sceneCode);

    //   if (scenePromise === undefined) return;

    //   scene = await scenePromise.build_scene();
    // } catch (error) {
    //   console.warn(error);
    //   // TODO
    //   return;
    // }

    this.onSceneLoaded(width, height);

    let rows = height - 1;
    let work = this.workers.map(async (worker, i) => {
      if (i != 0) await worker.compileScene(sceneCode);

      while (rows >= 0) {
        let row = rows;
        rows -= 1;

        let colors = await worker.compute(row);

        if (colors) this.onLineComputed(row, width, height, colors);
      }
    });

    await Promise.all(work);

    this.onSceneRendered();
  }
}
