import * as Components from "./pkg/wasm_react.js";
import * as React from "https://cdn.skypack.dev/react";

export * as React from "https://cdn.skypack.dev/react";

let components = {};

export function useRustState(defaultValue, onFree) {
  let [state, setState] = React.useState(() => [defaultValue()]);

  React.useEffect(() => () => onFree(state[0]), []);

  return [
    state[0],
    (mutator) =>
      setState((state) => {
        mutator(state[0]);
        return [state[0]];
      }),
  ];
}

export function registerComponent(name) {
  Object.assign(components, {
    [name]: ({ rustProps }) => Components[name].render(rustProps),
  });
}

export function getComponent(name) {
  return components[name];
}

export function cast(x) {
  return x;
}
