# wasm-react

[![GitHub](https://img.shields.io/badge/GitHub-Repo-lightgrey?logo=github)](https://github.com/yishn/wasm-react)
[![CI](https://github.com/yishn/wasm-react/actions/workflows/ci.yml/badge.svg)](https://github.com/yishn/wasm-react/actions/workflows/ci.yml)

WIP: WASM bindings for [React](https://reactjs.org/).

## Introduction

This library enables you to write and use React components in Rust, which then
can be exported to JS to be reused or rendered.

### Goals

- Provide Rust bindings for the public API of `react` as close to the original
  API as possible.
- Provide an ergonomic way to write components.
- Provide ways to interact with components written in JS.

### Non-Goals

- Provide bindings for any other library than `react`, e.g. `react-dom`.
- Provide a reimplementation of the reconciliation algorithm or another runtime.
- Performance

## Getting Started

Make sure you have Rust and Cargo installed. You can include `wasm-react` by
adding it to your `Cargo.toml`. Furthermore, if you want to expose your Rust
components to JS, you also need `wasm-bindgen` and install
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/).

```toml
[dependencies]
wasm-react = "0.1"
wasm-bindgen = "0.2"
```

### Creating a Component

First, you need to define a struct for the props of your component. To define
the render function, you need to implement the trait `Component` for your
struct:

```rust
use wasm_react::{h, c, Component, VNode};

struct Counter {
  counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    h!(div)
      .build(c![
        h!(p).build(c!["Counter: ", self.counter]),
        h!(button).build(c!["Increment"]),
      ])
  }
}
```

### Add State

You can use the `use_state()` hook to make your component stateful:

```rust
use wasm_react::{h, c, Component, VNode};
use wasm_react::hooks::use_state;

struct Counter {
  initial_counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    let counter = use_state(|| self.initial_counter);

    h!(div)
      .build(c![
        h!(p).build(c!["Counter: ", *counter.value()]),
        h!(button).build(c!["Increment"]),
      ])
  }
}
```

Note that according to the usual Rust rules, the state will be dropped when the
render function returns. `use_state()` will prevent that by tying the lifetime
of the state to the lifetime of the component, therefore _persisting_ the state
through the entire lifetime of the component.

### Add Event Handlers

To create an event handler, you have to keep the lifetime of the closure beyond
the render function as well, so JS can call it in the future. You can persist a
closure by using the `use_callback()` hook:

```rust
use wasm_react::{h, c, Component, VNode};
use wasm_react::hooks::{use_state, use_callback, Deps};

struct Counter {
  initial_counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    let counter = use_state(|| self.initial_counter);
    let handle_click = use_callback({
      let mut counter = counter.clone();

      move |_| counter.set(|c| c + 1)
    }, Deps::none());

    h!(div)
      .build(c![
        h!(p).build(c!["Counter: ", *counter.value()]),
        h!(button)
          .on_click(&handle_click)
          .build(c!["Increment"]),
      ])
  }
}
```

### Export Component for JS Consumption

First, you'll need [`wasm-pack`](https://rustwasm.github.io/wasm-pack/). You can
use the `export_component!` macro to export your Rust component for JS
consumption. Requirement is that your component implements
`TryFrom<JsValue, Error = JsValue>`.

```rust
use wasm_react::{h, c, export_component, Component, VNode};
use wasm_bindgen::JsValue;

struct Counter {
  initial_counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    /* ... */
    VNode::empty()
  }
}

struct App;

impl Component for App {
  fn render(&self) -> VNode {
    h!(div).build(c![
      Counter {
        initial_counter: 0,
      },
    ])
  }
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

export_component!(App);
```

Use `wasm-pack` to compile your Rust code into WASM:

```sh
$ wasm-pack build
```

Depending on your JS project structure, you may want to specify the `--target`
option, see [`wasm-pack` documentation](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target).

Assuming you use a bundler that supports JSX, and WASM imports in ES modules,
like Webpack, you can use:

```js
import React from "react";
import { createRoot } from "react-dom/client";

async function main() {
  const { WasmReact, App } = await import("./path/to/wasm-bindings.js");
  WasmReact.useReact(window.React); // Tell WasmReact to use your React runtime

  const root = createRoot(document.getElementById("root"));
  root.render(<App />);
}
```

If you use plain ES modules, you can do the following:

```sh
$ wasm-pack build --target web
```

```js
import "https://unpkg.com/react/umd/react.production.min.js";
import "https://unpkg.com/react-dom/umd/react-dom.production.min.js";
import init, { WasmReact, App } from "./path/to/wasm-bindings.js";

async function main() {
  await init(); // Need to load WASM first
  WasmReact.useReact(window.React); // Tell WasmReact to use your React runtime

  const root = ReactDOM.createRoot(document.getElementById("root"));
  root.render(React.createElement(App, {}));
}
```
