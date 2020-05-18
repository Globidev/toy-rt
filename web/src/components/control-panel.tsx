import React from "react";

import Split from "react-split";

import $ from "jquery";

import "prismjs/prism.js";
import "prismjs/themes/prism-twilight.css";
import "prismjs/components/prism-python.min.js";

import "jquery.terminal";
import "jquery.terminal/css/jquery.terminal.min.css";
import "jquery.terminal/js/prism.js";

import { WasmExecutor } from "../wasm-executor";
import { WorkerState, EvalResult } from "../worker";

import * as Comlink from "comlink";

interface IControlPanelProps {
  onEval: (source: string) => Promise<EvalResult>;
  onRunScript: () => Promise<EvalResult>;
  wasmExecutor: WasmExecutor;
}

type RenderState =
  | { kind: "none" }
  | { kind: "rendering" }
  | { kind: "rendered"; time: number };

interface IControlPanelState {
  workerStates: WorkerState[];
  renderState: RenderState;
}

export class ControlPanel extends React.Component<
  IControlPanelProps,
  IControlPanelState
> {
  termRef = React.createRef<HTMLDivElement>();
  state: IControlPanelState = {
    workerStates: [],
    renderState: { kind: "none" },
  };

  constructor(props: IControlPanelProps) {
    super(props);

    this.props.wasmExecutor.events.on("sceneLoaded", () => {
      this.setState({ ...this.state, renderState: { kind: "rendering" } });
    });

    this.props.wasmExecutor.events.on("sceneRendered", (time) => {
      this.setState({ ...this.state, renderState: { kind: "rendered", time } });
    });

    this.wireWorkers();
  }

  wireWorkers() {
    this.props.wasmExecutor.workers.forEach((worker, id) => {
      let statePromise: Promise<WorkerState> = worker.state; // Typescript issue: Promise<A> | Promise<B> can't be coerced to a Promise<A | B>
      statePromise.then((state) => this.changeWorkerState(id, state));

      worker.setOnStateChange(
        Comlink.proxy((state) => this.changeWorkerState(id, state))
      );
    });
  }

  changeWorkerState(id: number, state: WorkerState) {
    let workerStates = this.state.workerStates.slice(0);
    workerStates[id] = state;
    let newState = { ...this.state, workerStates };
    this.setState(newState);
  }

  componentDidMount() {
    const termElement = this.termRef.current;
    const self = this;

    if (termElement != null) {
      const term = $(termElement).terminal(
        async (source, term) => {
          let result = await self.props.onEval(source);
          switch (result.kind) {
            case "success":
              if (result.returnValue != "None") term.echo(result.returnValue);
              break;

            case "error":
              break;
          }
        },
        {
          greetings: "Toy RT v1.0",
          prompt: "> ",
          completion: () => self.props.wasmExecutor.getLocalNames(),
          clear: false,
          onInit: () => $.terminal.syntax("python"),
        }
      );

      this.props.wasmExecutor.events.on("evalError", (error) => {
        term.error(error);
      });

      term.disable();
    }
  }

  render() {
    return (
      <div className="control-panel">
        <Split
          direction="horizontal"
          sizes={[60, 40]}
          minSize={[100, 100]}
          gutterSize={6}
          className="split-panel"
        >
          <div className="terminal-container" ref={this.termRef}></div>
          <div className="status-container">
            <div className="wasm-container-title">Wasm Workers</div>
            <div className="workers-container">
              {this.state.workerStates.map((state, id) => {
                return (
                  <div key={id} className="worker-state-container">
                    <span>{`[W${id}] - `}</span>
                    {stateElement(state)}
                  </div>
                );
              })}
            </div>
            {renderStateElement(this.state.renderState)}
          </div>
        </Split>
      </div>
    );
  }
}

function stateElement(state: WorkerState): JSX.Element {
  switch (state.kind) {
    case "created":
      return <span className="status-initial">Created</span>;
    case "loading":
      return (
        <React.Fragment>
          <span className="status-working">Loading</span>
          <span> …</span>
        </React.Fragment>
      );
    case "loaded":
      return <span className="status-done">Loaded</span>;
    case "idle":
      return <span className="status-done">Idle</span>;
    case "compute":
      return (
        <React.Fragment>
          <span className="status-working">Rendering</span>
          <span> row </span>
          <span className="status-data">{state.row}</span>
          <span> …</span>
        </React.Fragment>
      );
    case "eval":
      return (
        <React.Fragment>
          <span className="status-working">Evaluating</span>
          <span> script …</span>
        </React.Fragment>
      );
  }
}

function renderStateElement(renderState: RenderState): JSX.Element {
  switch (renderState.kind) {
    case "none":
      return <div></div>;
    case "rendering":
      return <div className="render-time">Rendering …</div>;
    case "rendered":
      return (
        <div className="render-time">
          Render Time: {renderState.time.toFixed()} ms
        </div>
      );
  }
}
