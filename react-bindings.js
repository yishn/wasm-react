import { __JsComponentWrapper } from "./pkg/wasm_react.js";
import {
  createElement,
  useState,
  useEffect,
} from "https://cdn.skypack.dev/react";

export * as React from "https://cdn.skypack.dev/react";

let components = {};

export function createComponent(name, rustProps) {
  if (components[name] == null) {
    Object.assign(components, {
      [name]: ({ rustProps }) => __JsComponentWrapper.render(rustProps),
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
