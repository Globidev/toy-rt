import React from "react";

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

interface IControlPanelState {
  running: boolean;
  workerStates: WorkerState[];
}

export class ControlPanel extends React.Component<
  IControlPanelProps,
  IControlPanelState
> {
  termRef = React.createRef<HTMLDivElement>();
  term: JQueryTerminal | null = null;
  state: IControlPanelState = {
    running: false,
    workerStates: [],
  };

  constructor(props: IControlPanelProps) {
    super(props);

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
      this.term = $(termElement).terminal(
        async (source, term) => {
          if (this.state.running) {
            return;
          }
          term.freeze(true);
          this.setState({ running: true });
          let result = await self.props.onEval(source);
          this.setState({ running: false });
          term.freeze(false);
          switch (result.kind) {
            case "success":
              if (result.returnValue != "None") term.echo(result.returnValue);
              break;

            case "error":
              term.error(result.error);
              break;
          }
        },
        {
          greetings: "Toy RT v1.0",
          prompt: "> ",
          completion: () => self.props.wasmExecutor.getLocalNames(),
          clear: false,
        }
      );

      $.terminal.syntax("python");

      this.props.wasmExecutor.onEvalError = (error) => {
        this.term?.error(error);
      };
    }
  }

  render() {
    return (
      <div className="control-panel">
        <div style={{ display: "flex", height: "100%" }}>
          <div style={{ flex: 0.1, margin: "5px" }}>
            <button
              onClick={async () => {
                this.term?.freeze(true);
                this.setState({ running: true });
                await this.props.onRunScript();
                this.setState({ running: false });
                this.term?.freeze(false);
              }}
              disabled={this.state.running}
            >
              Run script
            </button>
            <div className="workers-container">
              {this.state.workerStates.map((state, id) => {
                return (
                  <div key={id} className="worker-state-container">
                    <span>{`[W${id}] - `}</span>
                    {StateElement(state)}
                  </div>
                );
              })}
            </div>
          </div>
          <div ref={this.termRef} style={{ height: "100%", flex: 1 }}></div>
        </div>
      </div>
    );
  }
}

function StateElement(state: WorkerState): JSX.Element {
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
          <span>…</span>
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
