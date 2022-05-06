mod builtin_components;
mod component;
mod react_bindings;
mod test;
mod vnode;

pub mod callback;
pub mod hooks;
pub mod props;

use callback::*;
use wasm_bindgen::prelude::*;

pub use builtin_components::*;
pub use component::*;
pub use vnode::*;

/// Contains all functions exported to JS by `wasm-react`.
#[wasm_bindgen]
pub struct WasmReact;

#[wasm_bindgen]
impl WasmReact {
  /// Set the React runtime that `wasm-react` should use.
  ///
  /// **Please** call this function before doing anything else and only once.
  ///
  /// # Example
  ///
  /// ```js
  /// import init, { WasmReact } from "./path/to/wasm-bindings.js";
  /// import React from "https://cdn.skypack.dev/react";
  ///
  /// await init();
  /// WasmReact.setReact(React);
  /// ```
  #[wasm_bindgen(js_name = setReact)]
  pub fn set_react(value: JsValue) {
    react_bindings::set_react(value);
  }
}

/// The Rust equivalent to `React.createElement`. Use [`h!`] for a more
/// convenient way to create HTML elements. To create Rust components, use
/// [`VNode::from()`].
pub fn create_element(
  typ: impl Into<JsValue>,
  props: impl Into<JsValue>,
  children: VNodeList,
) -> VNode {
  VNode(react_bindings::create_element(
    &typ.into(),
    &props.into(),
    &children.into(),
  ))
}
