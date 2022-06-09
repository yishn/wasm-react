#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

// This hack is needed to let the doctests run for our README file
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

mod builtin_components;
mod component;
mod context;
mod value_container;
mod macros;
mod marker;
mod react_bindings;
mod vnode;

pub mod callback;
pub mod hooks;
pub mod props;

use props::Props;
use wasm_bindgen::prelude::*;

pub use builtin_components::*;
pub use component::*;
pub use context::*;
pub use value_container::*;
pub use marker::*;
#[doc(hidden)]
pub use paste::paste;
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
  /// Calling this function the multiple times will result in no-ops.
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
/// [`VNode::from()`].
pub fn create_element(
  typ: &JsValue,
  props: &Props,
  children: VNodeList,
) -> VNode {
  VNode(react_bindings::create_element(
    typ,
    props.as_ref(),
    &if !children.empty() {
      children.into()
    } else {
      JsValue::undefined()
    },
  ))
}
