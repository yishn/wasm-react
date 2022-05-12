# wasm-react

WIP: WASM bindings for [React].

## Introduction

This library enables you to write and use [React] components in Rust, which then
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

[React]: https://reactjs.org/
