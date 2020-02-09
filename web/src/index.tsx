import '../public/style.css';

import React from "react";
import { render } from "react-dom";
import AceEditor from "react-ace-builds";
import "react-ace-builds/webpack-resolver-min";

// function onChange(newValue: any) {
//   console.log("change", newValue);
// }

export function main() {
  trt.setup_panic_hook();

  let source = window.localStorage.getItem("last-source") || `
  from trt import Sphere, Rect, Scene, Camera

red = (0.9, 0.3, 0.3)
green = (0.3, 0.9, 0.3)
blue = (0.3, 0.3, 0.9)
white = (0.7, 0.7, 0.7)

def scene():
    spheres = [
        Sphere(
            center=(150+ i * 150, 150 + i * 100, 150 + i * 100),
            radius=80,
            color=color,
        )
        for i, color in enumerate((red, green, blue))
    ]

    cornell_box = [
        # side walls
        Rect(y=(0, 555), z=(-1000, 555), x=555, color=red).flip_normals(),
        Rect(y=(0, 555), z=(-1000, 555), x=0, color=blue),

        # ceiling
        Rect(x=(0, 555), z=(-1000, 555), y=555, color=white).flip_normals(),
        # floor
        Rect(x=(0, 555), z=(-1000, 555), y=0, color=green),

        # front
        Rect(x=(0, 555), y=(0, 555), z=555, color=white).flip_normals(),
        # back
        Rect(x=(0, 555), y=(0, 555), z=-1000, color=white),
    ]

    return Scene(**{
        "world": cornell_box + spheres,
        "width": 800,
        "height": 800,
        "samples_per_px": 10,
        "rays_per_sample": 30,
        "camera": Camera(
            look_at=(300, 300, 0),
            look_from=(300, 300, -800),
        )
    })


  `;
  render(
    <AceEditor
      mode="python"
      theme="monokai"
      onChange={(s) => { source = s }}
      name="ace-editor-rendered"
      editorProps={{ $blockScrolling: true }}
      value={source}
      height="100%"
      width="100%"
    />,
    document.getElementById("editor")
  );

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
        case "ready":
          availableWorkers.add(i)
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

  runBtn.onclick = () => {
    window.localStorage.setItem("last-source", source)
    let scene = trt.Scene.new(source);
    canvas.setAttribute("width", scene.width().toString())
    canvas.setAttribute("height", scene.height().toString())

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
      }
    }

    let start = Date.now();
    dispatchWork()
  }

  const sliderValue = document.getElementById('worker-slider-value') as HTMLDivElement;
  slider.oninput = () => {
    sliderValue.innerHTML = `Workers: ${slider.value}`;
  }
}
