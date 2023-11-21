#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

// This hack is needed to let the doctests run for our README file
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

mod builtin_components;
mod callback;
mod component;
mod context;
mod macros;
mod value_container;
mod vnode;

pub mod hooks;
pub mod props;
#[doc(hidden)]
pub mod react_bindings;

use props::Props;
use wasm_bindgen::prelude::*;

pub use builtin_components::*;
pub use callback::*;
pub use component::*;
pub use context::*;
#[doc(hidden)]
pub use paste::paste;
pub use value_container::*;
pub use vnode::*;

/// Contains all functions exported to JS by `wasm-react`. These functions should
/// be called from JS only.
#[doc(hidden)]
#[wasm_bindgen]
pub struct WasmReact;

#[wasm_bindgen]
impl WasmReact {
  /// Set the React runtime that `wasm-react` should use.
  ///
  /// Calling this function multiple times will result in no-ops.
  ///
  /// # Example
  ///
  /// ```js
  /// import React from "react";
  /// import init, { WasmReact } from "./path/to/pkg/project.js";
  ///
  /// async function main() {
  ///   await init();
  ///   WasmReact.useReact(React);
  /// }
  ///
  /// main();
  /// ```
  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue) {
    react_bindings::use_react(value);
  }
}

/// The Rust equivalent to `React.createElement`. Use [`h!`] for a more
/// convenient way to create HTML element nodes. To create Rust components, use
/// [`Component::build()`].
pub fn create_element(typ: &JsValue, props: &Props, children: VNode) -> VNode {
  VNode::Single(react_bindings::create_element(
    typ,
    props.as_ref(),
    &children.into(),
  ))
}
