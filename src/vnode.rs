#![allow(non_snake_case)]

use js_sys::{Array, JsString};
use wasm_bindgen::JsValue;

/// Represents a node in the virtual DOM of React.
#[derive(Debug, Clone)]
pub enum VNode {
  #[doc(hidden)]
  Single(JsValue),
  #[doc(hidden)]
  List(Array),
}

impl VNode {
  /// Creates an empty node that doesn't render anything.
  pub fn new() -> VNode {
    VNode::Single(JsValue::null())
  }

  /// Adds the given node to the list.
  pub fn push(&mut self, node: &VNode) {
    match self {
      VNode::Single(x) => {
        *self = VNode::List({
          let arr = Array::new();

          if !x.is_null() {
            arr.push(x);
          }

          arr.push(node.as_ref());
          arr
        });
      }
      VNode::List(arr) => {
        arr.push(node.as_ref());
      }
    }
  }
}

impl Default for VNode {
  fn default() -> Self {
    Self::new()
  }
}

impl AsRef<JsValue> for VNode {
  fn as_ref(&self) -> &JsValue {
    match self {
      VNode::Single(x) => x,
      VNode::List(x) => x,
    }
  }
}

impl From<VNode> for JsValue {
  fn from(value: VNode) -> Self {
    match value {
      VNode::Single(x) => x,
      VNode::List(x) => x.into(),
    }
  }
}

impl<T: Into<VNode>> From<Option<T>> for VNode {
  fn from(value: Option<T>) -> Self {
    value.map(|value| value.into()).unwrap_or_default()
  }
}

impl Extend<VNode> for VNode {
  fn extend<T: IntoIterator<Item = VNode>>(&mut self, iter: T) {
    for node in iter.into_iter() {
      self.push(&node);
    }
  }
}

impl FromIterator<VNode> for VNode {
  fn from_iter<T: IntoIterator<Item = VNode>>(iter: T) -> Self {
    let mut result = Self::new();

    for node in iter.into_iter() {
      result.push(&node);
    }

    result
  }
}

macro_rules! impl_into_vnode {
  { $( $T:ty ),* $(,)? } => {
    $(
      impl From<$T> for VNode {
        fn from(value: $T) -> Self {
          VNode::Single(value.into())
        }
      }
    )*
  };
}

// Implement `Into<VNode>` for as many `Display` types as possible
impl_into_vnode! {
  &str, String, JsString,
  f32, f64,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
}

impl From<()> for VNode {
  fn from(_: ()) -> Self {
    VNode::new()
  }
}

macro_rules! impl_into_vnode_for_tuples {
  (@impl) => {};
  (@impl $( $x:ident ),+) => {
    impl<$( $x, )+> From<($( $x, )+)> for VNode
    where $( $x: Into<VNode>, )+
    {
      fn from(($( $x, )+): ($( $x, )+)) -> VNode {
        let mut result = VNode::new();
        $( result.push(&$x.into()); )+
        result
      }
    }

    impl_into_vnode_for_tuples!(@next $( $x ),+);
  };
  (@next $first:ident) => {};
  (@next $first:ident, $( $tt:tt )*) => {
    impl_into_vnode_for_tuples!(@impl $( $tt )*);
  };
  ($( $x:ident ),*) => {
    impl_into_vnode_for_tuples!(@impl $( $x ),*);
  }
}

impl_into_vnode_for_tuples!(
  A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
