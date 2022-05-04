import { __JsComponentWrapper } from "../../wasm_react.js";
import {
  createElement,
  useState,
  useEffect,
} from "https://cdn.skypack.dev/react";

export * as React from "https://cdn.skypack.dev/react";

let components = {};

export function getComponent(name) {
  if (components[name] == null) {
    // This curious construction is needed to ensure that the components show up
    // with their names correctly in the React Developer Tools
    Object.assign(components, {
      [name]: ({ rustProps }) => {
        // We need to free up the memory on Rust side whenever the old props are
        // replaced with new ones.
        useEffect(() => () => rustProps.free(), [rustProps]);

        return __JsComponentWrapper.render(rustProps);
      },
    });
  }

  return components[name];
}

export function createComponent(name, rustProps) {
  return createElement(getComponent(name), { rustProps });
}

export function useRustState(create, onFree) {
  // We only maintain a pointer to the state struct
  let [state, setState] = useState(() => ({ ptr: create() }));
  // Let Rust free up the memory whenever the component unmounts
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
