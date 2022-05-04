mod callback;
mod component;
mod react;
mod vnode;

use attr::Attr;
use wasm_bindgen::JsValue;

pub mod attr;
pub mod hooks;
pub mod test;

pub use callback::*;
pub use component::*;
pub use vnode::*;

pub fn create_element(
  typ: JsValue,
  props: Attr,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  VNode(react::create_element(
    typ,
    props.into(),
    children.into_iter().map(|c| JsValue::from(c)).collect(),
  ))
}

pub fn html(
  tag: &str,
  attr: Attr,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  create_element(tag.into(), attr, children)
}
