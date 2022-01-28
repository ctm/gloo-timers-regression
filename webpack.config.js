const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "gloo-timers-web.js",
      webassemblyModuleFilename: "gloo-timers-web.wasm"
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    experiments: {
      syncWebAssembly: true,
    },
    watch: false,
  };
}
