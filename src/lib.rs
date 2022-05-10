// #![warn(missing_docs)]
#![doc = include_str!("../README.md")]

// This hack is needed to let the doctests run for our README file
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

mod builtin_components;
mod component;
mod macros;
mod marker;
mod react_bindings;
mod test;
mod vnode;

pub mod callback;
pub mod hooks;
pub mod props;

use callback::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub use builtin_components::*;
pub use component::*;
pub use marker::*;
pub use vnode::*;

/// Contains all functions exported to JS by `wasm-react`. These functions should
/// be called from JS only.
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
  /// import React from "react";
  /// import init, { WasmReact } from "./path/to/wasm-bindings.js";
  ///
  /// async function main() {
  ///   await init();
  ///   WasmReact.useReact(React);
  /// }
  ///
  /// main();
  /// ```
  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue) -> Result<(), JsValue> {
    react_bindings::use_react(value)
  }
}

/// The Rust equivalent to `React.createElement`. Use [`h!`] for a more
/// convenient way to create HTML element nodes. To create Rust components, use
/// [`VNode::from()`].
pub fn create_element(
  typ: &JsValue,
  props: impl AsRef<JsValue>,
  children: VNodeList,
) -> VNode {
  VNode(react_bindings::create_element(
    typ,
    props.as_ref(),
    children.as_ref(),
  ))
}

#[derive(Debug)]
pub struct Context<T: 'static> {
  fallback_value: Rc<T>,
  js_context: JsValue,
}

impl<T: 'static> Clone for Context<T> {
  fn clone(&self) -> Self {
    Self {
      fallback_value: self.fallback_value.clone(),
      js_context: self.js_context.clone(),
    }
  }
}

pub fn create_context<T>(init: T) -> Context<T> {
  Context {
    fallback_value: Rc::new(init),
    js_context: react_bindings::create_context(&JsValue::undefined()),
  }
}
