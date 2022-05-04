mod callback;
mod component;
mod react_bindings;
mod test;
mod vnode;

pub mod hooks;
pub mod props;

use props::{Props, H};
use wasm_bindgen::prelude::*;

pub use callback::*;
pub use component::*;
pub use vnode::*;

/// Contains all functions exported to Javascript by `wasm-react`.
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

/// The Rust equivalent to `React.createElement`. Use [`h()`] for a more
/// convenient way to create HTML elements. To create Rust components, use
/// [`Component::into_vnode()`].
pub fn create_element(
  typ: &JsValue,
  props: impl Into<JsValue>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  VNode(react_bindings::create_element(
    typ,
    &props.into(),
    &children.into_iter().map(|c| JsValue::from(c)).collect(),
  ))
}

/// A convenience function to [`create_element()`] for creating HTML elements.
/// This returns a builder [`H`] which provides auto-completion for HTML
/// attributes and events.
///
/// # Example
///
/// ```
/// h("div")
///   .attr("id", "app")
///   .attr("className", "info")
///   .children([
///     h("h1").children(["Hello World!".into()])
///   ])
/// ```
pub fn h<'a>(tag: &'a str) -> H<'a> {
  H {
    tag,
    props: Props::new(),
  }
}
