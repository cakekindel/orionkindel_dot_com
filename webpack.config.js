const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './src/Index.bs.js',
  // If you ever want to use webpack during development, change 'production'
  // to 'development' as per webpack documentation. Again, you don't have to
  // use webpack or any other bundler during development! Recheck README if
  // you didn't know this
  mode: 'production',
  output: {
    path: path.join(__dirname, "bundleOutput"),
    filename: 'index.js',
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname + '/src/flow_field',
    }),
    new CopyPlugin({
        patterns: [
        { from: path.resolve(__dirname + '/static') }
      ]
    }),
  ]
}
