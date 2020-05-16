import ReactDOM from "react-dom";
import React from "react";

import Split from "react-split";

import { Canvas } from "./components/canvas";
import { GithubCorner } from "./components/github-corner";
import { Editor } from "./components/editor";
import { ControlPanel } from "./components/control-panel";

import { WasmExecutor } from "./wasm-executor";
import { EvalResult } from "./worker";

import demoCode from "../public/demo.py";

import "../public/style.css";

export function main() {
  const $root = document.getElementById("app");
  ReactDOM.render(<App />, $root);
}

interface IAppState {
  wasmExecutor: WasmExecutor;
  rendering: boolean;
  runningScript: boolean;
}

class App extends React.Component<{}, IAppState> {
  sceneCode = loadLastSource();
  state: IAppState = {
    wasmExecutor: new WasmExecutor(navigator.hardwareConcurrency),
    // wasmExecutor: new WasmExecutor(1),
    rendering: false,
    runningScript: false,
  };

  async componentDidMount() {
    await this.state.wasmExecutor.init();
  }

  async evalCode(source: string): Promise<EvalResult> {
    this.setState({ ...this.state, runningScript: true });
    let result = await this.state.wasmExecutor.eval(source);

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
    let result = await this.evalCode(this.sceneCode);
    return result;
  }

  onSceneCodeChanged(code: string) {
    saveLastSource(code);
    this.sceneCode = code;
  }

  render() {
    const wasmExecutor = this.state.wasmExecutor;
    const runBtnText = this.state.rendering ? "STOP ■" : "RUN ⯈";

    return (
      <React.Fragment>
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
            </div>
            <Editor
              initialSource={this.sceneCode}
              onChange={(code) => this.onSceneCodeChanged(code)}
              onRunScript={() => this.evalScript()}
            />
          </div>
          <ControlPanel
            onEval={(code) => this.evalCode(code)}
            onRunScript={() => this.evalScript()}
            wasmExecutor={wasmExecutor}
          />
        </Split>
        <Canvas wasmExecutor={wasmExecutor} />
      </React.Fragment>
    );
  }
}

const LAST_SOURCE_STORAGE_KEY = "last-source";

function loadLastSource() {
  return window.localStorage.getItem(LAST_SOURCE_STORAGE_KEY) || demoCode;
}

function saveLastSource(source: string) {
  window.localStorage.setItem(LAST_SOURCE_STORAGE_KEY, source);
}
