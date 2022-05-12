# wasm-react [![GitHub](https://img.shields.io/badge/GitHub-Repo-lightgrey?logo=github)](https://github.com/yishn/wasm-react) [![CI](https://github.com/yishn/wasm-react/actions/workflows/ci.yml/badge.svg)](https://github.com/yishn/wasm-react/actions/workflows/ci.yml)

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
adding it to your `Cargo.toml`.

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
    h!(div[."counter"])
      .build(c![
        h!(p).build(c!["Counter: ", self.counter]),
        h!(button).build(c!["Increment"]),
      ])
  }
}
```
