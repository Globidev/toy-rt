import React from "react";

import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/theme/monokai.css";
import "codemirror/mode/python/python.js";
import "codemirror/addon/comment/comment.js";

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
        lineNumbers: true,
        indentUnit: 4,
        indentWithTabs: false,
        extraKeys: {
          Tab: (cm) => cm.execCommand("indentMore"),
          "Shift-Tab": (cm) => cm.execCommand("indentLess"),
          "Ctrl-Enter": (_cm) => this.props.onRunScript(),
          "Ctrl-/": (cm) => cm.execCommand("toggleComment"),
        },
      });

      cm.on("change", (self, changes) => {
        let code = cm.getValue();
        this.props.onChange(code);
      });

      setTimeout(() => {
        cm.refresh();
        cm.focus();
      }, 0);

      this.cm = cm;
    }
  }

  render() {
    return <div className="editor-container" ref={this.editorRef}></div>;
  }
}
