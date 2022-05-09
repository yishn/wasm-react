import { __WasmReact_ComponentWrapper } from "../../../wasm_react.js";

let components = {};

export let React = undefined;

export function useReact(value) {
  if (React == null) {
    React = value;
  } else {
    throw new Error("React runtime is already set");
  }
}

function registerRustComponent(name) {
  if (components[name] == null) {
    // All Rust components have the same implementation in JS, but we need to
    // define them separately, so the names show up correctly in the React
    // Developer Tools.
    //
    // It shouldn't be a problem if two Rust components share the same name.
    Object.assign(components, {
      [name]: (props = {}) => {
        if (props.component instanceof __WasmReact_ComponentWrapper) {
          let component = props.component;

          // We need to free up the memory on Rust side whenever the old props
          // are replaced with new ones.
          React.useEffect(() => () => component.free(), [component]);

          return __WasmReact_ComponentWrapper.render(component);
        } else {
          throw new Error("Cannot create non-Rust component");
        }
      },
    });
  }
}

export function getRustComponent(name) {
  registerRustComponent(name);
  return components[name];
}

export function createRustComponent(name, props) {
  return React.createElement(getRustComponent(name), props);
}

export function createBuiltinComponent(name, props, children) {
  return React.createElement(React[name], props, children);
}

export function useRustState() {
  // This only returns a function that can trigger a component rerender
  let [, setState] = React.useState(() => []);

  return () => setState([]);
}

export function useRustRef(create) {
  let ref = React.useRef(null);

  if (ref.current == null) {
    // Create ref struct if called for the first time and maintain a pointer
    ref.current = create();
  }

  return ref.current;
}

export function useUnmountHandler(handler) {
  let firstRenderRef = React.useRef(true);

  // The callback `handler` has to be called exactly one time so the Rust memory
  // of its corresponding closure will be freed. If this function has been
  // called for the first time, this `useEffect` will ensure that `handler` will
  // be called with `true` when the component unmounts. But if we're rerendering,
  // we have to call `handler` with `false` manually, so the closure can be
  // dropped on Rust side.
  React.useEffect(() => {
    firstRenderRef.current = false;

    return () => handler(true);
  }, []);

  if (!firstRenderRef.current) {
    handler(false);
  }
}
