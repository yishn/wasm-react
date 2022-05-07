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
  // This only returns a function that can trigger a component update
  let [, setState] = React.useState(() => []);

  return () => setState([]);
}

export function useRustRef(create, onFree) {
  let ref = React.useRef(null);

  // Create ref struct if called for the first time
  if (ref.current == null) {
    // We only maintain a pointer to the ref struct
    let ptr = create();
    ref.current = ptr;
  } else {
    // The closure `onFree` has to be called exactly one time so the Rust memory
    // of its corresponding closure will be freed. If this function has been
    // called for the first time, the `useEffect` below will ensure that `onFree`
    // will be called when the component unmounts. But if not, we have to call
    // `onFree` manually, so the closure can be dropped on Rust side.
    onFree(null);
  }

  // Let Rust free up the memory whenever the component unmounts
  React.useEffect(() => () => onFree(ref.current), []);

  return ref.current;
}
