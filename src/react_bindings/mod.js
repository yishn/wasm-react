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
