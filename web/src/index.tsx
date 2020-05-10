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
}

class App extends React.Component<{}, IAppState> {
  sceneCode = initialSource();
  state: IAppState = {
    wasmExecutor: new WasmExecutor(navigator.hardwareConcurrency),
  };

  async componentDidMount() {
    await this.state.wasmExecutor.init();
  }

  async evalCode(source: string): Promise<EvalResult> {
    let result = await this.state.wasmExecutor.eval(source);

    if (result.kind == "success" && result.sceneDimensions !== null) {
      await this.state.wasmExecutor.render(result.sceneDimensions, source);
    }

    return result;
  }

  async evalScript() {
    return await this.evalCode(this.sceneCode);
  }

  render() {
    let wasmExecutor = this.state.wasmExecutor;
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
            return { height: `calc(${size}% - ${gutterSize + 8}px)` };
          }}
        >
          <Editor
            initialSource={this.sceneCode}
            onChange={(code) => (this.sceneCode = code)}
            onRunScript={() => this.evalScript()}
          />
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

function initialSource() {
  return window.localStorage.getItem("last-source-v2") || demoCode;
}
