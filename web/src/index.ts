export function main() {
  trt.setup_panic_hook();

  const runBtn = document.getElementById('run-btn') as HTMLButtonElement;

  runBtn.onclick = run

  const sliderValue = document.getElementById('worker-slider-value') as HTMLDivElement;
  const slider = document.getElementById('worker-slider') as HTMLInputElement;
  slider.oninput = () => {
    sliderValue.innerHTML = `Workers: ${slider.value}`;
  }
}

function run() {
  const codeArea = document.getElementById('code-area') as HTMLTextAreaElement;
  const slider = document.getElementById('worker-slider') as HTMLInputElement;

  const canvas = document.getElementById('canvas') as HTMLCanvasElement;
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

  const MAX_X = 300;
  const MAX_Y = 300;

  ctx.clearRect(0, 0, MAX_X, MAX_Y)

  const drawLine = (y: number, colors: Uint32Array) => {
    for (let x = 0; x < MAX_X; ++x) {
      const r = (colors[x] & 0x00FF0000) >> 16;
      const g = (colors[x] & 0x0000FF00) >> 8;
      const b = (colors[x] & 0x000000FF) >> 0;

      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`
      ctx.fillRect(x, MAX_Y - y, 1, 1)
    }
  }

  let y = MAX_Y - 1;

  const workers: Worker[] = [];
  const availableWorkers: Set<number> = new Set();

  for (let i = 0; i < parseInt(slider.value); i++) {
    const worker = new Worker('./worker.js')

    worker.onmessage = (e) => {
      switch (e.data.type) {
        case "ready":
          worker.postMessage({ type: 'scene', code: codeArea.value })
          availableWorkers.add(i)
          break

        case "row":
          const colors = e.data.colors;
          const row = e.data.row;
          drawLine(row, colors)
          availableWorkers.add(i)
          break

        default:
          break
      }
    }

    workers.push(worker)
  }

  function dispatchWork() {
    if (y >= 0) {
      for (let workerId of availableWorkers) {
        workers[workerId].postMessage({ type: 'compute', row: y })
        y -= 1
        if (y < 0) { break }
      }
      availableWorkers.clear()
      setTimeout(dispatchWork, 0);
    }
  }

  dispatchWork()
}
