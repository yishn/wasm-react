export let React = window.React;

export function useReact(value) {
  if (React == null) {
    React = value;
  }
}

export function createElement(name, props, children) {
  if (!Array.isArray(children)) children = [children];
  return React.createElement(name, props, ...children);
}
