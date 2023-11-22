# wasm-react 🦀⚛️

[![GitHub](https://img.shields.io/badge/GitHub-Repo-lightgrey?logo=github)](https://github.com/yishn/wasm-react)
[![crates.io](https://img.shields.io/crates/v/wasm-react)](https://crates.io/crates/wasm-react)
[![CI](https://github.com/yishn/wasm-react/actions/workflows/ci.yml/badge.svg)](https://github.com/yishn/wasm-react/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/wasm-react)](https://docs.rs/wasm-react/)

WASM bindings for [React].

## Introduction

This library enables you to write and use React components in Rust, which then
can be exported to JS to be reused or rendered.

### Why React?

React is one of the most popular UI framework for JS with a thriving community
and lots of libraries written for it. Standing on the shoulder of giants, you
will be able to write complex frontend applications with Rust.

### Goals

- Provide Rust bindings for the public API of `react` as close to the original
  API as possible, but with Rust in mind.
- Provide an ergonomic way to write components.
- Provide ways to interact with components written in JS.

### Non-Goals

- Provide bindings for any other library than `react`, e.g. `react-dom`.
- Reimplementation of the reconciliation algorithm or runtime.
- Emphasis on performance.

## Getting Started

Make sure you have Rust and Cargo installed. You can install `wasm-react` with
cargo. Furthermore, if you want to expose your Rust components to JS, you also
need `wasm-bindgen` and install [`wasm-pack`].

```sh
$ cargo add wasm-react
$ cargo add wasm-bindgen@0.2
```

### Creating a Component

First, you need to define a struct for the props of your component. To define
the render function, you need to implement the trait `Component` for your
struct:

```rust
use wasm_react::{h, Component, VNode};

struct Counter {
  counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    h!(div)
      .build((
        h!(p).build(("Counter: ", self.counter)),
        h!(button).build("Increment"),
      ))
  }
}
```

### Add State

You can use the `use_state()` hook to make your component stateful:

```rust
use wasm_react::{h, Component, VNode};
use wasm_react::hooks::use_state;

struct Counter {
  initial_counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    let counter = use_state(|| self.initial_counter);

    let vnode = h!(div)
      .build((
        h!(p).build(("Counter: ", *counter.value())),
        h!(button).build("Increment"),
      ));
    vnode
  }
}
```

Note that according to the usual Rust rules, the state will be dropped when the
render function returns. `use_state()` will prevent that by tying the lifetime
of the state to the lifetime of the component, therefore _persisting_ the state
through the entire lifetime of the component.

### Add Event Handlers

To create an event handler, you pass a `Callback` created from a Rust closure.
You can use the helper macro `clones!` to clone-capture the environment more
ergonomically.

```rust
use wasm_react::{h, clones, Component, Callback, VNode};
use wasm_react::hooks::{use_state, Deps};

struct Counter {
  initial_counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    let message = use_state(|| "Hello World!");
    let counter = use_state(|| self.initial_counter);

    let value = h!(div)
      .build((
        h!(p).build(("Counter: ", *counter.value())),

        h!(button)
          .on_click(&Callback::new({
            clones!(message, mut counter);
            move |_| {
              println!("{}", message.value());
              counter.set(|c| c + 1);
            }
          }))
          .build("Increment"),

        h!(button)
          .on_click(&Callback::new({
            clones!(mut counter);
            move |_| counter.set(|c| c - 1)
          }))
          .build("Decrement"),
      ));
    value
  }
}
```

### Export Components for JS Consumption

First, you'll need [`wasm-pack`]. You can use `export_components!` to export
your Rust component for JS consumption. Requirement is that your component
implements `TryFrom<JsValue, Error = JsValue>`.

```rust
use wasm_react::{h, export_components, Component, VNode};
use wasm_bindgen::JsValue;

struct Counter {
  initial_counter: i32,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    /* … */
    VNode::new()
  }
}

struct App;

impl Component for App {
  fn render(&self) -> VNode {
    h!(div).build((
      Counter {
        initial_counter: 0,
      }
      .build(),
    ))
  }
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

export_components! { App }
```

Use `wasm-pack` to compile your Rust code into WASM:

```sh
$ wasm-pack build
```

Depending on your JS project structure, you may want to specify the `--target`
option, see
[`wasm-pack` documentation](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target).

Assuming you use a bundler that supports JSX and WASM imports in ES modules like
Webpack, you can use:

```js
import React from "react";
import { createRoot } from "react-dom/client";

async function main() {
  const { WasmReact, App } = await import("./path/to/pkg/project.js");
  WasmReact.useReact(React); // Tell wasm-react to use your React runtime

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
import init, { WasmReact, App } from "./path/to/pkg/project.js";

async function main() {
  await init(); // Need to load WASM first
  WasmReact.useReact(window.React); // Tell wasm-react to use your React runtime

  const root = ReactDOM.createRoot(document.getElementById("root"));
  root.render(React.createElement(App, {}));
}
```

### Import Components for Rust Consumption

You can use `import_components!` together with `wasm-bindgen` to import JS
components for Rust consumption. First, prepare your JS component:

```js
// /.dummy/myComponents.js
import "https://unpkg.com/react/umd/react.production.min.js";

export function MyComponent(props) {
  /* … */
}
```

Make sure the component uses the same React runtime as specified for
`wasm-react`. Afterwards, use `import_components!`:

```rust
use wasm_react::{h, import_components, Component, VNode};
use wasm_bindgen::prelude::*;

import_components! {
  #[wasm_bindgen(module = "/.dummy/myComponents.js")]

  MyComponent
}

struct App;

impl Component for App {
  fn render(&self) -> VNode {
    h!(div).build((
      MyComponent::new()
        .attr("prop", &"Hello World!".into())
        .build(()),
    ))
  }
}
```

### Passing Down Non-Copy Props

Say you define a component with the following struct:

```rust
use std::rc::Rc;

struct TaskList {
  tasks: Vec<Rc<str>>
}
```

You want to include `TaskList` in a container component `App` where `tasks` is
managed by a state:

```rust
use std::rc::Rc;
use wasm_react::{h, Component, VNode};
use wasm_react::hooks::{use_state, State};

struct TaskList {
  tasks: Vec<Rc<str>>
}

impl Component for TaskList {
  fn render(&self) -> VNode {
    /* … */
    VNode::default()
  }
}

struct App;

impl Component for App {
  fn render(&self) -> VNode {
    let tasks: State<Vec<Rc<str>>> = use_state(|| vec![]);

    h!(div).build((
      TaskList {
        tasks: todo!(), // Oops, `tasks.value()` does not fit the type
      }
      .build(),
    ))
  }
}
```

Changing the type of `tasks` to fit `tasks.value()` doesn't work, since
`tasks.value()` returns a non-`'static` reference while component structs can
only contain `'static` values. You can clone the underlying `Vec`, but this
introduces unnecessary overhead. In this situation you might think you can
simply change the type of `TaskList` to a `State`:

```rust
use std::rc::Rc;
use wasm_react::{h, Component, VNode};
use wasm_react::hooks::{use_state, State};

struct TaskList {
  tasks: State<Vec<Rc<str>>>
}
```

This works as long as the prop `tasks` is guaranteed to come from a state. But
this assumption may not hold. You might want to pass on `Rc<Vec<Rc<str>>>` or
`Memo<Vec<Rc<str>>>` instead in the future or somewhere else. To be as generic
as possible, you can use `PropContainer`:

```rust
use std::rc::Rc;
use wasm_react::{h, Component, PropContainer, VNode};
use wasm_react::hooks::{use_state, State};

struct TaskList {
  tasks: PropContainer<Vec<Rc<str>>>
}

impl Component for TaskList {
  fn render(&self) -> VNode {
    /* Do something with `self.tasks.value()`… */
    VNode::default()
  }
}

struct App;

impl Component for App {
  fn render(&self) -> VNode {
    let tasks: State<Vec<Rc<str>>> = use_state(|| vec![]);

    h!(div).build((
      TaskList {
        // Cloning `State` has low cost as opposed to cloning the underlying
        // `Vec`.
        tasks: tasks.clone().into(),
      }
      .build(),
    ))
  }
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[react]: https://reactjs.org/
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack/
