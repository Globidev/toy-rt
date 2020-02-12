import("trt")
  .then(wasm => {
    wasm.setup_panic_hook();
    var scene: import('trt').Scene | null = null
    const vm = wasm.PythonVM.new();

    self.postMessage({ type: "ready" })
    self.onmessage = (e: any) => {
      switch (e.data.type) {
        case "scene":
          scene = vm.eval_scene(e.data.code)
          break

        case "compute":
          if (scene == null) { return }
          const row = e.data.row
          const colors = scene.row_color(row);
          self.postMessage({ type: 'row', colors, row, height: scene.height() })
          break

        default: break
      }
    }
  })
  .catch(err => console.error("Error importing wasm:", err));
