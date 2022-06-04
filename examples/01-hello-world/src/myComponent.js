import "https://unpkg.com/react/umd/react.production.min.js";

export function MyComponent(props) {
  return React.createElement("h2", {}, props.text);
}
