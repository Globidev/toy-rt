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
      {
        test: /\.js$/,
        loader: require.resolve("@open-wc/webpack-import-meta-loader"),
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
  plugins: [
    new CopyWebpackPlugin([
      "public/index.html",
      "../trt-wasm/pkg/trt_wasm_bg.wasm",
    ]),
  ],
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
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.js$/,
        loader: require.resolve("@open-wc/webpack-import-meta-loader"),
      },
    ],
  },
  mode: "development",
};

module.exports = [browserConfig, workerConfig];
