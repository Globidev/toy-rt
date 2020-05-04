const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

const browserConfig = {
  entry: "./src/bootstrap.ts",

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.(svg|png|md)$/,
        use: ["file-loader"],
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.py$/,
        use: ["raw-loader"],
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js", ".wasm"],
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  mode: "development",
  plugins: [new CopyWebpackPlugin(["public/index.html"])],
  devServer: {
    compress: true,
    inline: true,
    port: "8082",
    allowedHosts: ["manjaro"],
  },
};

const workerConfig = {
  entry: "./src/worker.ts",
  target: "webworker",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "worker.js",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  // resolve: {
  //   extensions: ['.tsx', '.ts', '.js']
  // },
  mode: "development",
};

module.exports = [browserConfig, workerConfig];
