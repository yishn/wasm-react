use crate::{props::Props, react_bindings, Component, ComponentWrapper};
use wasm_bindgen::JsValue;

/// Represents a node in the virtual DOM of React.
#[derive(Clone)]
pub struct VNode(pub(crate) JsValue);

impl From<VNode> for JsValue {
  fn from(value: VNode) -> Self {
    value.0
  }
}

impl<T: Component + 'static> From<T> for VNode {
  fn from(value: T) -> Self {
    VNode(react_bindings::create_component(
      stringify!(T),
      Props::new()
        .insert("key", value.key())
        .insert("component", ComponentWrapper(Box::new(value)))
        .into(),
    ))
  }
}

impl<'a> From<&'a str> for VNode {
  fn from(value: &'a str) -> Self {
    VNode(value.into())
  }
}

macro_rules! impl_into_vnode {
  ($($T:ty),*$(,)?) => {
    $(
      impl From<$T> for VNode {
        fn from(value: $T) -> Self {
          VNode(value.to_string().into())
        }
      }
    )*
  };
}

// Implement `Into<VNode>` for as many `Display` types as possible
impl_into_vnode! {
  String, char,
  f32, f64,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
}
