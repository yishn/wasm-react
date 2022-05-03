use std::fmt::Display;
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct VNode(pub(crate) JsValue);

impl From<VNode> for JsValue {
  fn from(value: VNode) -> Self {
    value.0
  }
}

impl<T> From<T> for VNode
where
  T: Display + Into<JsValue>,
{
  fn from(value: T) -> Self {
    VNode(value.into())
  }
}
