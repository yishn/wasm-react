const components = {};

export let React = undefined;

export function useReact(value) {
  if (React == null) {
    React = value;
  }
}

export function createElement(name, props, children = []) {
  return React.createElement(name, props, ...children);
}

function renderRustComponent(props) {
  // `component` is a `ComponentWrapper` or `MemoComponentWrapper`
  let component = props.component;

  // We need to free up the memory on Rust side whenever the old props
  // are replaced with new ones.
  React.useEffect(() => () => component.free(), [component]);

  return component.render();
}

function getRustComponent(name) {
  if (components[name] == null) {
    // All Rust components have the same implementation in JS, but we need to
    // define them separately, so that React can distinguish them as different
    // components, and also so the names show up correctly in the React
    // Developer Tools.
    Object.assign(components, {
      [name]: (props = {}) => renderRustComponent(props),
    });
  }

  return components[name];
}

export function createRustComponent(name, key, component) {
  return React.createElement(getRustComponent(name), {
    key,
    component,
  });
}

function getRustMemoComponent(name) {
  const memoName = `wasm_react::Memo<${name}>`;

  if (components[memoName] == null) {
    Object.assign(components, {
      [memoName]: React.memo(getRustComponent(name), (prevProps, nextProps) => {
        // `component` is a `MemoComponentWrapper`
        const equal = prevProps.component.eq(nextProps.component);

        if (equal) {
          // Since rerender is going to be prevented, we need to dispose of
          // `nextProps` manually.
          nextProps.component.free();
        }

        return equal;
      }),
    });
  }

  return components[name];
}

export function createRustMemoComponent(name, key, component) {
  return React.createElement(getRustMemoComponent(name), {
    key,
    component,
  });
}

export function useRustRef(create, handler) {
  let ref = React.useRef(null);

  if (ref.current == null) {
    // Create ref struct if called for the first time.

    ref.current = { ptr: create() };
  } else {
    // The callback `handler` has to be called exactly one time so the Rust
    // memory of its corresponding closure will be freed. If this function has
    // been called for the first time, the `useEffect` below will ensure that
    // `handler` will be called when the component unmounts.
    //
    // Otherwise, we have to call `handler` manually, so the closure can be
    // dropped on Rust side.

    handler(false, ref.current.ptr);
  }

  React.useEffect(() => () => handler(true, ref.current.ptr), []);

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

export function castToUsize(value) {
  return value;
}
