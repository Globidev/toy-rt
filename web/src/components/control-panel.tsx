import React from "react";

import $ from "jquery";

import "jquery.terminal";
import "jquery.terminal/css/jquery.terminal.min.css";

interface IControlPanelProps {
  onEval: (source: string) => Promise<string>;
  onRunScript: () => Promise<void>;
}

interface IControlPanelState {
  running: boolean;
}

export class ControlPanel extends React.Component<
  IControlPanelProps,
  IControlPanelState
> {
  termRef = React.createRef<HTMLDivElement>();
  state: IControlPanelState = { running: false };

  componentDidMount() {
    const termElement = this.termRef.current;
    const self = this;

    if (termElement != null) {
      $(termElement).terminal(
        async (source, term) => {
          try {
            let res = await self.props.onEval(source);
            term.echo(res);
          } catch (error) {
            term.error(error);
          }
        },
        {
          greetings: "Toy RT v1.0",
          prompt: "> ",
        }
      );
    }
  }

  render() {
    return (
      <div className="control-panel">
        <div style={{ display: "flex", height: "100%" }}>
          <div style={{ flex: 0.1, margin: "5px" }}>
            <button
              onClick={async () => {
                this.setState({ running: true });
                await this.props.onRunScript();
                this.setState({ running: false });
              }}
              disabled={this.state.running}
            >
              Run script
            </button>
          </div>
          <div ref={this.termRef} style={{ height: "100%", flex: 1 }}></div>
        </div>
      </div>
    );
  }
}
