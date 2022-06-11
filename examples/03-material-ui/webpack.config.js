const { resolve } = require("path");

module.exports = {
  entry: "./js/main.js",
  output: {
    path: resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  experiments: {
    asyncWebAssembly: true,
  },
};
