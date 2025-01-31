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
let ownerRef;
let tmpOwnerRef;

export function createRustComponent(name, component, extraProps) {
  rustComponents[name] ??= Object.assign(
    (props) => {
      // Get `ComponentWrapper` struct
      const component = props.component;

      // Set up owners
      ownerRef = React.useRef(null);

      if (ownerRef.current == null) {
        ownerRef.current = component.newOwner();
      }

      tmpOwnerRef = React.useRef(null);
      const oldTmpOwnerRef = tmpOwnerRef.current;
      tmpOwnerRef.current = component.newOwner();

      // Render
      const result = component.render(props.children);

      ownerRef = undefined;
      tmpOwnerRef = undefined;

      setTimeout(() => oldTmpOwnerRef?.free());

      return result;
    },
    { displayName: name }
  );

  return createElement(rustComponents[name], {
    ...extraProps,
    component,
  });
}

export function getOwner(callback) {
  if (ownerRef != null) {
    callback(ownerRef.current);
  }
}

export function getTmpOwner(callback) {
  if (tmpOwnerRef != null) {
    callback(tmpOwnerRef.current);
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

export function useEffect(f, dep) {
  React.useEffect(f, [dep]);
}

export function useLayoutEffect(f, dep) {
  React.useLayoutEffect(f, [dep]);
}

export function useInsertionEffect(f, dep) {
  React.useInsertionEffect(f, [dep]);
}

export function useTransition(callback) {
  const [isPending, startTransition] = React.useTransition();
  callback(isPending, startTransition);
}
