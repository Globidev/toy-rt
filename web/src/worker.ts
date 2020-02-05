import("trt")
  .then(wasm => {
    var scene: import('trt').Scene | null = null

    self.postMessage({ type: "ready" })
    self.onmessage = (e: any) => {
      switch (e.data.type) {
        case "scene":
          scene = wasm.Scene.new(e.data.code)
          break

        case "compute":
          if (scene == null) { return }
          const row = e.data.row
          const colors = scene.row_color(row);
          self.postMessage({ type: 'row', colors, row })
          break

        default: break
      }
    }
  })
  .catch(err => console.error("Error importing wasm:", err));
