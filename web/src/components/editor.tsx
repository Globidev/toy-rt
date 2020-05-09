import React from "react";

import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/theme/monokai.css";
import "codemirror/mode/python/python.js";

interface IEditorProps {
  initialSource: string;
  onChange: (code: string) => void;
  onRunScript: () => void;
}

export class Editor extends React.Component<IEditorProps> {
  editorRef = React.createRef<HTMLDivElement>();
  cm: CodeMirror.Editor | null = null;

  componentDidMount() {
    let editor = this.editorRef.current;
    if (editor !== null) {
      let source = this.props.initialSource;

      const cm = CodeMirror(editor, {
        value: source,
        mode: "python",
        theme: "monokai",
      });

      cm.on("change", (self, changes) => {
        let code = cm.getValue();
        this.props.onChange(code);
      });

      cm.setOption("extraKeys", {
        "Ctrl-Enter": (_cm) => {
          this.props.onRunScript();
        },
      });

      this.cm = cm;
    }
  }

  render() {
    return (
      <div className="editor-container">
        <div ref={this.editorRef}></div>
      </div>
    );
  }
}
