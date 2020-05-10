import ReactDOM from "react-dom";
import React from "react";

import { Canvas } from "./components/canvas";
import { GithubCorner } from "./components/github-corner";
import { Editor } from "./components/editor";
import { ControlPanel } from "./components/control-panel";

import { WasmExecutor } from "./wasm-executor";

import demoCode from "../public/demo.py";

import "../public/style.css";

function initialSource() {
  return window.localStorage.getItem("last-source-v2") || demoCode;
}

interface IAppState {
  wasmExecutor: WasmExecutor | null;
}

class App extends React.Component<{}, IAppState> {
  sceneCode = initialSource();
  state: IAppState = { wasmExecutor: null };

  async componentDidMount() {
    let wasmExecutor = await WasmExecutor.new(4);
    this.setState({ wasmExecutor });
  }

  async evalCode(source: string) {
    return (await this.state.wasmExecutor?.eval(source)) || "";
  }

  async evalScene() {
    await this.state.wasmExecutor?.render(this.sceneCode);
  }

  render() {
    let wasmExecutor = this.state.wasmExecutor;
    return (
      <React.Fragment>
        <GithubCorner />
        <div>
          <Editor
            initialSource={this.sceneCode}
            onChange={(code) => (this.sceneCode = code)}
            onRunScript={() => this.evalScene()}
          />
          <ControlPanel
            onEval={(code) => this.evalCode(code)}
            onRunScript={() => this.evalScene()}
          />
        </div>
        {wasmExecutor && <Canvas wasmExecutor={wasmExecutor} />}
      </React.Fragment>
    );
  }
}

export function main() {
  trt.setup_panic_hook();

  const $root = document.getElementById("app");
  ReactDOM.render(<App />, $root);
}
