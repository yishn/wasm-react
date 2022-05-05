use crate::{props::Props, react_bindings, Component, ComponentWrapper};
use js_sys::Array;
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
      T::name(),
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

/// Represents a list of nodes in the virtual DOM of React.
#[derive(Default, Clone)]
pub struct VNodeList(Array);

impl VNodeList {
  /// Creates a new, empty list.
  pub fn new() -> Self {
    Self(Array::new())
  }

  /// Adds the given node to the list.
  pub fn push(&self, node: VNode) {
    self.0.push(&node.into());
  }
}

impl From<VNodeList> for JsValue {
  fn from(value: VNodeList) -> Self {
    value.0.into()
  }
}

impl From<VNodeList> for VNode {
  fn from(value: VNodeList) -> Self {
    VNode(value.into())
  }
}

impl Extend<VNode> for VNodeList {
  fn extend<T: IntoIterator<Item = VNode>>(&mut self, iter: T) {
    for node in iter.into_iter() {
      self.push(node);
    }
  }
}

impl FromIterator<VNode> for VNodeList {
  fn from_iter<T: IntoIterator<Item = VNode>>(iter: T) -> Self {
    let result = Self::new();

    for node in iter.into_iter() {
      result.push(node);
    }

    result
  }
}
