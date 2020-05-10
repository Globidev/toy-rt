declare module "*.py" {
  const content: string;
  export default content;
}

declare module "react-split" {
  interface SplitOptions {
    sizes?: number[];
    minSize?: number[] | number;
    gutterSize?: number;
    snapOffset?: number;
    direction?: "horizontal" | "vertical";
    cursor?: "col-resize" | "row-resize";
    gutter?: (index: number, direction: string) => HTMLElement;
    elementStyle?: (
      dimension: string,
      elementSize: number,
      gutterSize: number
    ) => any;
    gutterStyle?: (dimension: string, gutterSize: number) => any;
    onDrag?: () => void;
    onDragStart?: () => void;
    onDragEnd?: () => void;
  }

  interface SplitObject {
    setSizes: (sizes: number[]) => void;
    getSizes: () => number[];
    collapse: (index: number) => void;
    destroy: () => void;
  }

  type SplitOpts = SplitOptions & { className?: string };

  class Split extends React.Component<SplitOpts> {}

  export = Split;
}
