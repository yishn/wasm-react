export let React = window.React;
export let Runtime = null;

export function useReact(value) {
  if (React == null) {
    React = value;
  }
}

export function useRuntime(value) {
  if (Runtime == null) {
    Runtime = value;
  }
}

export function createElement(type, props) {
  const key = props.key;
  delete props.key;
  return Runtime.jsx(type, props, key);
}

const rustComponents = {};

export function createRustComponent(name, component, extraProps) {
  rustComponents[name] ??= Object.assign(
    (props) => {
      // Get the `ComponentWrapper`
      const componentWrapper = props.component;

      return componentWrapper.render(props.children);
    },
    { displayName: name }
  );

  return createElement(rustComponents[name], {
    ...extraProps,
    component,
  });
}

let ownerRef;

export function useOwnerSetup(init) {
  ownerRef = React.useRef(null);

  if (ownerRef.current == null) {
    ownerRef.current = init();
  }
}

export function getOwner(callback) {
  if (ownerRef != null) {
    callback(ownerRef.current);
  }
}

export function useRef(init, callback) {
  const ref = React.useRef(null);

  if (ref.current == null) {
    ref.current = init();
  }

  callback(ref.current);
}

export function useUpdate() {
  const [, set] = React.useState({});
  return () => set({});
}
