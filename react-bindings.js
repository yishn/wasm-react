import * as Components from "./pkg/wasm_react.js";
import * as React from "https://cdn.skypack.dev/react";

let components = {};

export function getComponent(name) {
  if (components[name] == null) {
    Object.assign(components, {
      [name]: ({ rustProps }) => Components[name].render(rustProps),
    });
  }

  return components[name];
}

export function createElement(...args) {
  return React.createElement(...args);
}

export function useRustState(defaultValue, onFree) {
  let [state, setState] = React.useState(() => [defaultValue()]);

  React.useEffect(() => () => onFree(state[0]), []);

  return [
    state[0],
    (mutator) =>
      setState((state) => {
        mutator();
        return [state[0]];
      }),
  ];
}
