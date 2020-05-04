import "codemirror/lib/codemirror.css";
import "../public/style.css";
//@ts-ignore ????????
import demoCode from "../public/demo.py";

import CodeMirror from "codemirror";
import "codemirror/mode/python/python.js";
import "codemirror/theme/monokai.css";

export function main() {
  trt.setup_panic_hook();

  const canvasContainer = document.getElementsByClassName(
    "canvas-container"
  )[0] as HTMLDivElement;

  var pos1 = 0,
    pos2 = 0,
    pos3 = 0,
    pos4 = 0;

  function dragMouseDown(e: MouseEvent) {
    e = e || window.event;
    e.preventDefault();
    // get the mouse cursor position at startup:
    pos3 = e.clientX;
    pos4 = e.clientY;
    document.onmouseup = closeDragElement;
    // call a function whenever the cursor moves:
    document.onmousemove = elementDrag;
  }

  canvasContainer.onmousedown = dragMouseDown;

  function elementDrag(e: MouseEvent) {
    e = e || window.event;
    e.preventDefault();
    // calculate the new cursor position:
    pos1 = pos3 - e.clientX;
    pos2 = pos4 - e.clientY;
    pos3 = e.clientX;
    pos4 = e.clientY;
    // set the element's new position:
    canvasContainer.style.top = canvasContainer.offsetTop - pos2 + "px";
    canvasContainer.style.left = canvasContainer.offsetLeft - pos1 + "px";
  }

  function closeDragElement() {
    /* stop moving when mouse button is released:*/
    document.onmouseup = null;
    document.onmousemove = null;
  }

  const pythonVM = trt.PythonVM.new();

  let source = window.localStorage.getItem("last-source-v2") || demoCode;
  const run = async () => {
    const el = document.getElementById("traceback") as HTMLDivElement;
    el.innerText = "";

    window.localStorage.setItem("last-source-v2", source);

    let scene;
    try {
      const scenePromise = pythonVM.eval(source);

      if (scenePromise === undefined) return;

      scene = await scenePromise.build_scene();
    } catch (error) {
      el.innerText = error;
      return;
    }

    canvas.setAttribute("width", scene.width().toString());
    canvas.setAttribute("height", scene.height().toString());

    canvas.classList.add("canvas-building");

    for (let worker of workers) {
      worker.postMessage({ type: "scene", code: source });
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    let y = scene.height() - 1;

    function dispatchWork() {
      if (y >= 0) {
        let currentlyAvailableWorkers = [...availableWorkers];
        for (let workerId of currentlyAvailableWorkers) {
          if (workerId >= parseInt(slider.value)) {
            continue;
          }
          workers[workerId].postMessage({ type: "compute", row: y });
          availableWorkers.delete(workerId);
          y -= 1;
          if (y < 0) {
            break;
          }
        }
        setTimeout(dispatchWork, 0);
      } else {
        console.log(Date.now() - start);
        canvas.classList.remove("canvas-building");
      }
    }

    let start = Date.now();
    dispatchWork();
  };

  const editor = CodeMirror(document.getElementById("editor") as HTMLElement, {
    value: source,
    mode: "python",
    theme: "monokai",
  });

  editor.on("change", (self, changes) => {
    source = editor.getValue();
  });

  editor.setOption("extraKeys", {
    "Ctrl-Enter": (_cm) => {
      run();
    },
  });

  // console.log(foo)

  const runBtn = document.getElementById("run-btn") as HTMLButtonElement;

  // const codeArea = document.getElementById('code-area') as HTMLTextAreaElement;
  const slider = document.getElementById("worker-slider") as HTMLInputElement;

  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
  let gradient = ctx.createLinearGradient(0, 0, canvas.width, canvas.height);
  gradient.addColorStop(0, "cyan");
  gradient.addColorStop(0.5, "orange");
  gradient.addColorStop(1, "purple");
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  const drawLine = (
    y: number,
    width: number,
    height: number,
    colors: Uint32Array
  ) => {
    for (let x = 0; x < width; ++x) {
      const r = (colors[x] & 0x00ff0000) >> 16;
      const g = (colors[x] & 0x0000ff00) >> 8;
      const b = (colors[x] & 0x000000ff) >> 0;

      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
      ctx.fillRect(x, height - y, 1, 1);
    }
  };

  const workers: Worker[] = [];
  const availableWorkers: Set<number> = new Set();
  for (let i = 0; i < 1; i++) {
    const worker = new Worker("./worker.js");

    worker.onmessage = (e) => {
      switch (e.data.type) {
        case "error":
          let loadingCtrls = document.getElementsByClassName(
            "loading-ctrls"
          )[0] as HTMLDivElement;
          loadingCtrls.innerText = e.data.why;
          break;

        case "ready":
          availableWorkers.add(i);
          if (availableWorkers.size == 1) {
            let ctrls = document.getElementsByClassName(
              "ctrls"
            )[0] as HTMLDivElement;
            let loadingCtrls = document.getElementsByClassName(
              "loading-ctrls"
            )[0] as HTMLDivElement;
            ctrls.style.display = "block";
            loadingCtrls.style.display = "none";
          }
          break;

        case "row":
          const colors = e.data.colors;
          const row = e.data.row;
          drawLine(row, colors.length, e.data.height, colors);
          availableWorkers.add(i);
          break;

        default:
          break;
      }
    };

    workers.push(worker);
  }

  runBtn.onclick = run;

  const sliderValue = document.getElementById(
    "worker-slider-value"
  ) as HTMLDivElement;
  slider.oninput = () => {
    sliderValue.innerHTML = `Workers: ${slider.value}`;
  };
}
