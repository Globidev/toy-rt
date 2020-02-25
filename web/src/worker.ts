import("trt")
  .then(wasm => {
    console.log("WORKER LOADED")

    wasm.setup_panic_hook();

    let scene: import('trt').Scene | null = null
    let backlog: any | null = null

    const compute = (scene: import('trt').Scene, e: any) => {
      const row = e.data.row
      const colors = scene.row_color(row);
      self.postMessage({ type: 'row', colors, row, height: scene.height() })
    }

    const vm = wasm.PythonVM.new();

    self.postMessage({ type: "ready" })

    self.onmessage = async (e: any) => {
      switch (e.data.type) {
        case "scene":
          scene = null
          const scenePromise = vm.eval(e.data.code);
          if (scenePromise === undefined)
            return;
          scene = await scenePromise.build_scene() as import('trt').Scene
          if (backlog !== null) {
            compute(scene, backlog)
          }
          break

        case "compute":
          if (scene == null) {
            backlog = e
            return
          }
          compute(scene, e)
          break

        default: break
      }
    }
  })
  .catch(err => {
    console.error("Error importing wasm:", err);
    self.postMessage({ type: 'error', why: `Error importing wasm: ${err}` })
  });
