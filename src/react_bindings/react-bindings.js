let components = {};

export let React = undefined;

export function useReact(value) {
  if (React == null) {
    React = value;
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

function getRustComponent(name, typeId) {
  let key = `${typeId} (${name.replace(/(\w+::)+/g, "")})`;

  if (components[key] == null) {
    // All Rust components have the same implementation in JS, but we need to
    // define them separately, so that React can distinguish them as different
    // components, and also so the names show up correctly in the React
    // Developer Tools.
    Object.assign(components, {
      [key]: (props = {}) => renderRustComponent(props),
    });
  }

  return components[key];
}

export function createRustComponent(name, typeId, key, component) {
  return React.createElement(getRustComponent(name, typeId), {
    key,
    component,
  });
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
