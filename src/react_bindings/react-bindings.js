let components = {};

export let React = undefined;

export function useReact(value) {
  if (React == null) {
    React = value;
  } else {
    throw new Error("React runtime is already set");
  }
}

function renderRustComponent(props) {
  // `component` is a `ComponentWrapper`
  let component = props.component;

  // We need to free up the memory on Rust side whenever the old props
  // are replaced with new ones.
  React.useEffect(() => () => component.free(), [component]);

  return component.render();
}

function registerRustComponent(name) {
  if (components[name] == null) {
    // All Rust components have the same implementation in JS, but we need to
    // define them separately, so the names show up correctly in the React
    // Developer Tools.
    //
    // It shouldn't be a problem if two Rust components share the same name.
    Object.assign(components, {
      [name]: (props = {}) => renderRustComponent(props),
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

export function useRustRef(create, handler) {
  let ref = React.useRef(null);

  if (ref.current == null) {
    // Create ref struct if called for the first time.

    ref.current = { ptr: create(), dropped: false };
  } else {
    // The callback `handler` has to be called exactly one time so the Rust
    // memory of its corresponding closure will be freed. If this function has
    // been called for the first time, the `useEffect` below will ensure that
    // `handler` will be called when the component unmounts.
    //
    // Otherwise, we have to call `handler` manually, so the closure can be
    // dropped on Rust side.

    handler(false, ref.current.ptr, ref.current);
  }

  React.useEffect(() => () => handler(true, ref.current.ptr, ref.current), []);

  return ref.current;
}

export function useRustState() {
  // This only returns a function that can trigger a component rerender
  let [, setState] = React.useState(() => []);

  return () => setState([]);
}

export function useRustEffect(effect, dep) {
  React.useEffect(effect, [dep]);
}

export function useRustLayoutEffect(effect, dep) {
  React.useLayoutEffect(effect, [dep]);
}

export function childrenToArray(children) {
  return React.Children.toArray(children);
}

export function cast(value) {
  return value;
}
