mod callback;
mod component;
mod react_bindings;
mod vnode;

pub mod attr;
pub mod hooks;
pub mod test;

use wasm_bindgen::prelude::*;

pub use callback::*;
pub use component::*;
pub use vnode::*;

#[wasm_bindgen]
pub struct WasmReact;

#[wasm_bindgen]
impl WasmReact {
  #[wasm_bindgen(js_name = setReact)]
  pub fn set_react(value: JsValue) {
    react_bindings::set_react(value);
  }
}

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

pub fn html(
  tag: &str,
  attr: impl Into<JsValue>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  create_element(&tag.into(), attr, children)
}
