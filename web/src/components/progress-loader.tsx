import React from "react";
import { Line } from "rc-progress";

interface IProgressLoaderProps {
  progressPercent: number;
}

export const ProgressLoader = (props: IProgressLoaderProps) => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      justifyContent: "center",
      height: "100%",
    }}
  >
    <h1>Fetching Webassembly</h1>
    <div style={{ height: "20%", width: "20%" }}>
      <Line
        percent={props.progressPercent}
        strokeColor="#36d7b7"
        strokeWidth={2}
        trailWidth={2}
      ></Line>
    </div>
  </div>
);
