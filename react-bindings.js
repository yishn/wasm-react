import * as Components from "./pkg/wasm_react.js";

export * as React from "https://cdn.skypack.dev/react";

let components = {};

export function registerComponent(name) {
  Object.assign(components, {
    [name]: (props) => Components[name].render(props),
  });
}

export function getComponent(name) {
  return components[name];
}
