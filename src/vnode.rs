use crate::{react_bindings, Component, ComponentWrapper};
use js_sys::Array;
use std::any::type_name;
use wasm_bindgen::{JsCast, JsValue};

/// Represents a node in the virtual DOM of React.
#[derive(Clone)]
pub struct VNode(pub(crate) JsValue);

impl VNode {
  /// An empty node that doesn't render anything.
  pub fn empty() -> VNode {
    VNode(JsValue::null())
  }
}

impl Default for VNode {
  fn default() -> Self {
    Self::empty()
  }
}

impl AsRef<JsValue> for VNode {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}

impl From<VNode> for JsValue {
  fn from(value: VNode) -> Self {
    value.0
  }
}

impl<T: Component> From<T> for VNode {
  fn from(value: T) -> Self {
    VNode(react_bindings::create_rust_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<T>(),
      &value.key().into(),
      &ComponentWrapper(Box::new(value)).into(),
    ))
  }
}

impl<T: Into<VNode>> From<Option<T>> for VNode {
  fn from(value: Option<T>) -> Self {
    value.map(|value| value.into()).unwrap_or_default()
  }
}

impl<'a> From<&'a str> for VNode {
  fn from(value: &'a str) -> Self {
    VNode(value.into())
  }
}

impl<'a> From<&'a String> for VNode {
  fn from(value: &'a String) -> Self {
    VNode(value.into())
  }
}

macro_rules! impl_into_vnode {
  ($($T:ty),*$(,)?) => {
    $(
      impl From<$T> for VNode {
        fn from(value: $T) -> Self {
          VNode(value.into())
        }
      }
    )*
  };
}

// Implement `Into<VNode>` for as many `Display` types as possible
impl_into_vnode! {
  String,
  f32, f64,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
}

/// Represents a list of nodes in the virtual DOM of React.
///
/// Use the [`c!`](crate::c!) macro to build a [`VNodeList`] more conveniently.
/// You can also collect an iterator of [`VNode`] into a [`VNodeList`]:
///
/// ```
/// # use wasm_react::*;
/// #
/// # fn f() -> VNodeList {
/// vec!["first item", "second item", "third item"]
///   .iter()
///   .map(|&x| h!(li).build(c![x]))
///   .collect::<VNodeList>()
/// # }
/// ```
#[derive(Default, Clone)]
pub struct VNodeList {
  empty: bool,
  arr: Array,
}

impl VNodeList {
  /// Creates a new, empty list.
  pub fn new() -> Self {
    Self {
      empty: true,
      arr: Array::new(),
    }
  }

  /// Returns whether the list is empty or not.
  pub fn empty(&self) -> bool {
    self.empty
  }

  /// Adds the given node to the list.
  pub fn push(&mut self, node: &VNode) {
    self.empty = false;
    self.arr.push(node.as_ref());
  }

  /// Adds the given node list to the list.
  pub fn extend(&mut self, iter: impl Iterator<Item = VNode>) {
    for node in iter {
      self.push(&node);
    }
  }
}

impl AsRef<JsValue> for VNodeList {
  fn as_ref(&self) -> &JsValue {
    &self.arr
  }
}

impl From<VNodeList> for JsValue {
  fn from(value: VNodeList) -> Self {
    value.arr.into()
  }
}

impl TryFrom<JsValue> for VNodeList {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    let arr = if value.is_instance_of::<Array>() {
      value.dyn_into::<Array>()?
    } else {
      react_bindings::children_to_array(&value)?
    };

    Ok(VNodeList {
      empty: arr.length() == 0,
      arr,
    })
  }
}

impl Extend<VNode> for VNodeList {
  fn extend<T: IntoIterator<Item = VNode>>(&mut self, iter: T) {
    for node in iter.into_iter() {
      self.push(&node);
    }
  }
}

impl FromIterator<VNode> for VNodeList {
  fn from_iter<T: IntoIterator<Item = VNode>>(iter: T) -> Self {
    let mut result = Self::new();

    for node in iter.into_iter() {
      result.push(&node);
    }

    result
  }
}
