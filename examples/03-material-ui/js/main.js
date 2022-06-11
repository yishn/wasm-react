import React from "react";
import ReactDOM from "react-dom/client";

async function main() {
  const { WasmReact, App } = await import("../pkg/material_ui.js");
  WasmReact.useReact(React);

  const root = ReactDOM.createRoot(document.getElementById("root"));
  root.render(React.createElement(App));
}

main();
