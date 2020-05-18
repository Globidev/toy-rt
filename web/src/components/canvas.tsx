import React from "react";
import Draggable from "react-draggable";
import { WasmExecutor } from "../wasm-executor";

interface ICanvasProps {
  wasmExecutor: WasmExecutor;
}

interface ICanvasState {
  rendering: boolean;
  width: number;
  height: number;
}

export class Canvas extends React.Component<ICanvasProps, ICanvasState> {
  canvasRef = React.createRef<HTMLCanvasElement>();
  state: ICanvasState = { rendering: false, width: 500, height: 500 };

  componentDidMount() {
    const canvas = this.canvasRef.current;
    if (canvas !== null) {
      const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

      ctx.fillStyle = defaultCanvasFilling(ctx, canvas.width, canvas.height);
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const initialRender = loadLastRender();
      if (initialRender !== null) {
        const img = new Image();
        img.src = initialRender;
        img.onload = () => {
          ctx.clearRect(0, 0, canvas.width, canvas.height);
          this.setState({
            ...this.state,
            width: img.width,
            height: img.height,
          });
          ctx.drawImage(img, 0, 0);
        };
      }

      let executor = this.props.wasmExecutor;

      executor.events.on("sceneLoaded", (width, height) => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        this.setState({ rendering: true, width, height });
      });

      executor.events.on("lineComputed", (y, width, height, colors) => {
        for (let x = 0; x < width; ++x) {
          const r = (colors[x] & 0x00ff0000) >> 16;
          const g = (colors[x] & 0x0000ff00) >> 8;
          const b = (colors[x] & 0x000000ff) >> 0;

          ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
          ctx.fillRect(x, height - y, 1, 1);
        }
      });

      executor.events.on("sceneRendered", () => {
        this.setState({ ...this.state, rendering: false });
        saveLastRender(canvas.toDataURL());
      });
    }
  }

  render() {
    return (
      <div className="canvas-container">
        <Draggable positionOffset={{ x: "calc(90vw - 500px)", y: "10vh" }}>
          <div
            className={
              this.state.rendering ? "active-border" : "inactive-border"
            }
            style={{
              width: this.state.width + 4,
              height: this.state.height + 4,
            }}
          >
            <canvas
              width={this.state.width}
              height={this.state.height}
              ref={this.canvasRef}
              id="canvas"
            ></canvas>
          </div>
        </Draggable>
      </div>
    );
  }
}

function defaultCanvasFilling(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number
) {
  const gradient = ctx.createLinearGradient(0, 0, width, height);
  gradient.addColorStop(0, "cyan");
  gradient.addColorStop(0.5, "orange");
  gradient.addColorStop(1, "purple");
  return gradient;
}

const LAST_RENDER_STORAGE_KEY = "last-render-data";

function loadLastRender() {
  return window.localStorage.getItem(LAST_RENDER_STORAGE_KEY);
}

function saveLastRender(renderData: string) {
  window.localStorage.setItem(LAST_RENDER_STORAGE_KEY, renderData);
}
