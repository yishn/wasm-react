import * as Components from "./pkg/wasm_react.js";
import { useState, useEffect } from "https://cdn.skypack.dev/react";

export * as React from "https://cdn.skypack.dev/react";

let components = {};

export function getComponent(name) {
  if (components[name] == null) {
    Object.assign(components, {
      [name]: ({ rustProps }) => Components[name].render(rustProps),
    });
  }

  return components[name];
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
