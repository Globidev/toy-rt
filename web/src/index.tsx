import ReactDOM from "react-dom";
import React from "react";

import Split from "react-split";

import { Canvas } from "./components/canvas";
import { GithubCorner } from "./components/github-corner";
import { Editor } from "./components/editor";
import { ControlPanel } from "./components/control-panel";
import { ProgressLoader } from "./components/progress-loader";

import { WasmExecutor } from "./wasm-executor";
import { EvalResult } from "./worker";

import { scenes } from "./assets/scenes";

import "../public/style.css";
import { EvalMode } from "trt";

export function main() {
  const $root = document.getElementById("app");
  ReactDOM.render(<App />, $root);
}

interface IAppState {
  wasmExecutor: WasmExecutor;
  rendering: boolean;
  runningScript: boolean;
  loadingWasm: LoadingWasmState;
}

type LoadingWasmState =
  | { kind: "loading"; percent: number }
  | { kind: "loaded" };

class App extends React.Component<{}, IAppState> {
  sceneCode = loadLastSource();
  editorRef = React.createRef<Editor>();
  state: IAppState = {
    wasmExecutor: new WasmExecutor(navigator.hardwareConcurrency),
    // wasmExecutor: new WasmExecutor(1),
    rendering: false,
    runningScript: false,
    loadingWasm: { kind: "loading", percent: 0 },
  };

  async componentDidMount() {
    const resp = await fetch("trt_wasm_bg.wasm");
    const reader = resp.body?.getReader();
    const length = 21_311_358;
    console.log(length);
    if (reader == undefined) return;

    let wasmData = new Uint8Array();
    for (;;) {
      const result = await reader.read();
      if (result.done) {
        break;
      }
      const newData = new Uint8Array(result.value.length + wasmData.length);
      newData.set(wasmData, 0);
      newData.set(result.value, wasmData.length);
      wasmData = newData;
      this.setState({
        ...this.state,
        loadingWasm: {
          kind: "loading",
          percent: (wasmData.length / length) * 100,
        },
      });
    }

    this.setState({ ...this.state, loadingWasm: { kind: "loaded" } });

    await this.state.wasmExecutor.init(wasmData);
  }

  async evalCode(source: string, mode: EvalMode): Promise<EvalResult> {
    this.setState({ ...this.state, runningScript: true });
    let result = await this.state.wasmExecutor.eval(source, mode);

    if (result.kind == "success" && result.sceneDimensions !== null) {
      if (this.state.rendering) {
        return { kind: "error", error: "A scene is already being rendered" };
      }

      this.setState({ ...this.state, rendering: true });
      await this.state.wasmExecutor.render(result.sceneDimensions, source);
      this.setState({ ...this.state, rendering: false });
    }

    this.setState({ ...this.state, runningScript: false });
    return result;
  }

  async evalScript() {
    let result = await this.evalCode(this.sceneCode, EvalMode.Verbose);
    return result;
  }

  onSceneCodeChanged(code: string) {
    saveLastSource(code);
    this.sceneCode = code;
  }

  render() {
    const wasmExecutor = this.state.wasmExecutor;
    const runBtnText = this.state.rendering ? "STOP ■" : "RUN ⯈";

    if (this.state.loadingWasm.kind == "loading") {
      return (
        <ProgressLoader progressPercent={this.state.loadingWasm.percent} />
      );
    }

    return (
      <div className="main">
        <GithubCorner />
        <Split
          sizes={[80, 20]}
          minSize={[200, 100]}
          direction="vertical"
          className="split-main"
          gutterSize={6}
          elementStyle={(dim, size, gutterSize) => {
            return { height: `calc(${size}% - ${gutterSize + 4}px)` };
          }}
        >
          <div className="top-container">
            <div className="top-controls">
              <button
                className="run-btn"
                onClick={async () => {
                  if (!this.state.rendering) await this.evalScript();
                  else wasmExecutor.cancelRender();
                }}
                disabled={this.state.runningScript && !this.state.rendering}
              >
                {runBtnText}
              </button>
              <select
                className="load-model-select"
                onChange={(e) => {
                  const selectedSceneKey = e.target.value;
                  const scene = scenes[selectedSceneKey];
                  if (scene !== undefined && this.editorRef.current) {
                    this.sceneCode = scene;
                    this.editorRef.current.cm?.setValue(scene);
                  }
                  e.target.selectedIndex = 0;
                }}
              >
                <option>Load model…</option>
                {Object.keys(scenes).map((sceneName, idx) => {
                  return (
                    <option value={sceneName} key={idx}>
                      {sceneName}
                    </option>
                  );
                })}
              </select>
            </div>
            <Editor
              initialSource={this.sceneCode}
              onChange={(code) => this.onSceneCodeChanged(code)}
              onRunScript={() => this.evalScript()}
              ref={this.editorRef}
            />
          </div>
          <ControlPanel
            onEval={(code) => this.evalCode(code, EvalMode.Verbose)}
            onRunScript={() => this.evalScript()}
            wasmExecutor={wasmExecutor}
          />
        </Split>
        <Canvas wasmExecutor={wasmExecutor} />
      </div>
    );
  }
}

const LAST_SOURCE_STORAGE_KEY = "last-source";

function loadLastSource() {
  return (
    window.localStorage.getItem(LAST_SOURCE_STORAGE_KEY) ||
    scenes["gophers versus the world"]
  );
}

function saveLastSource(source: string) {
  window.localStorage.setItem(LAST_SOURCE_STORAGE_KEY, source);
}
