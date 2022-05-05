mod callback;
mod component;
mod react_bindings;
mod test;
mod vnode;

pub mod hooks;
pub mod props;

use wasm_bindgen::prelude::*;

pub use callback::*;
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
  typ: &JsValue,
  props: impl Into<JsValue>,
  children: VNodeList,
) -> VNode {
  VNode(react_bindings::create_element(
    typ,
    &props.into(),
    &children.into(),
  ))
}

/// Can be used to create a [React fragment][fragment].
///
/// [fragment]: https://reactjs.org/docs/fragments.html
///
/// # Example
///
/// ```
/// Fragment.build(children![
///   h!(h1).build(children!["Hello World!"]),
///   h!(div).build(children!["No wrapper element"]),
/// ])
/// ```
pub struct Fragment;

impl Fragment {
  /// Returns a [`VNode`] which represents a [React fragment][fragment].
  ///
  /// [fragment]: https://reactjs.org/docs/fragments.html
  pub fn build(&self, children: VNodeList) -> VNode {
    VNode(react_bindings::create_fragment(&children.into()))
  }
}
