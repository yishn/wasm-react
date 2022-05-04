mod callback;
mod component;
mod react_bindings;
mod test;
mod vnode;

pub mod props;
pub mod hooks;

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

/// The Rust equivalent to `React.createElement`. Use [`html()`] for a more
/// convenient way to create HTML elements.
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
///
/// # Example
///
/// ```
/// html("div", Attr::new().id("app"), [
///   html("h1", None, ["Hello World!".into()])
/// ])
/// ```
pub fn html(
  tag: &str,
  attr: impl Into<JsValue>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  create_element(&tag.into(), attr, children)
}
