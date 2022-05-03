import * as Components from "./pkg/wasm_react.js";
import {
  createElement,
  useState,
  useEffect,
} from "https://cdn.skypack.dev/react";

export * as React from "https://cdn.skypack.dev/react";

let components = {};

export function renderComponent(name, rustProps) {
  if (components[name] == null) {
    Object.assign(components, {
      [name]: ({ rustProps }) => Components[name].render(rustProps),
    });
  }

  return createElement(components[name], { rustProps });
}

export function useRustState(create, onFree) {
  let [state, setState] = useState(() => ({ ptr: create() }));

  useEffect(() => () => onFree(state.ptr), []);

  return [
    state.ptr,
    (mutator) =>
      setState((state) => {
        mutator();
        return { ...state };
      }),
  ];
}

export function cast(x) {
  return x;
}
