import '../public/style.css';
//@ts-ignore ????????
import demoCode from '../public/demo.py';

import React from "react";
import { render } from "react-dom";
import AceEditor from "react-ace-builds";
import "react-ace-builds/webpack-resolver-min";

export function main() {
  trt.setup_panic_hook();

  const pythonVM = trt.PythonVM.new();

  let source = window.localStorage.getItem("last-source") || demoCode;
  const run = () => {
    window.localStorage.setItem("last-source", source)

    let scene;
    try {
      scene = pythonVM.eval_scene(source);
    } catch (error) {
      const el = document.getElementById('traceback') as HTMLDivElement;
      el.innerText = error
      return
    }

    canvas.setAttribute("width", scene.width().toString())
    canvas.setAttribute("height", scene.height().toString())

    canvas.classList.add('canvas-building')

    for (let worker of workers) {
      worker.postMessage({ type: 'scene', code: source })
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    let y = scene.height() - 1;

    function dispatchWork() {
      if (y >= 0) {
        let currentlyAvailableWorkers = [...availableWorkers];
        for (let workerId of currentlyAvailableWorkers) {
          if (workerId >= parseInt(slider.value)) {
            continue
          }
          workers[workerId].postMessage({ type: 'compute', row: y })
          availableWorkers.delete(workerId)
          y -= 1
          if (y < 0) { break }
        }
        setTimeout(dispatchWork, 0);
      } else {
        console.log(Date.now() - start)
        canvas.classList.remove('canvas-building')
      }
    }

    let start = Date.now();
    dispatchWork()
  };

  render(
    <AceEditor
      mode="python"
      theme="monokai"
      onChange={(s) => { source = s }}
      name="ace-editor-rendered"
      editorProps={{ $blockScrolling: true }}
      value={source}
      height="80%"
      width="100%"
      ref={(e) => {
        if (e == null) return
        // @ts-ignore
        const editor = e.editor;
        editor.commands.addCommand({
          name: "Run",
          bindKey: { win: "Ctrl-Enter", mac: "Command-Enter" },
          exec: run
        });
      }}
      enableBasicAutocompletion={true}
    />,
    document.getElementById("editor")
  );

  // console.log(foo)

  const runBtn = document.getElementById('run-btn') as HTMLButtonElement;

  // const codeArea = document.getElementById('code-area') as HTMLTextAreaElement;
  const slider = document.getElementById('worker-slider') as HTMLInputElement;

  const canvas = document.getElementById('canvas') as HTMLCanvasElement;
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

  const drawLine = (y: number, width: number, height: number, colors: Uint32Array) => {
    for (let x = 0; x < width; ++x) {
      const r = (colors[x] & 0x00FF0000) >> 16;
      const g = (colors[x] & 0x0000FF00) >> 8;
      const b = (colors[x] & 0x000000FF) >> 0;

      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`
      ctx.fillRect(x, height - y, 1, 1)
    }
  }

  const workers: Worker[] = []
  const availableWorkers: Set<number> = new Set();
  for (let i = 0; i < 8; i++) {
    const worker = new Worker('./worker.js')

    worker.onmessage = (e) => {
      switch (e.data.type) {
        case "error":
          let loadingCtrls = document.getElementsByClassName('loading-ctrls')[0] as HTMLDivElement;
          loadingCtrls.innerText = e.data.why
          break

        case "ready":
          availableWorkers.add(i)
          if (availableWorkers.size == 8) {
            let ctrls = document.getElementsByClassName('ctrls')[0] as HTMLDivElement;
            let loadingCtrls = document.getElementsByClassName('loading-ctrls')[0] as HTMLDivElement;
            ctrls.style.display = "block";
            loadingCtrls.style.display = "none";
          }
          break

        case "row":
          const colors = e.data.colors;
          const row = e.data.row;
          drawLine(row, colors.length, e.data.height, colors)
          availableWorkers.add(i)
          break

        default:
          break
      }
    }

    workers.push(worker)
  }


  runBtn.onclick = run

  const sliderValue = document.getElementById('worker-slider-value') as HTMLDivElement;
  slider.oninput = () => {
    sliderValue.innerHTML = `Workers: ${slider.value}`;
  }
}
