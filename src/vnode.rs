#![allow(non_snake_case)]

use crate::{c, react_bindings};
use js_sys::{Array, JsString};
use wasm_bindgen::{JsCast, JsValue};

/// Represents a node in the virtual DOM of React.
#[derive(Debug, Clone)]
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

impl<T: Into<VNode>> From<Option<T>> for VNode {
  fn from(value: Option<T>) -> Self {
    value.map(|value| value.into()).unwrap_or_default()
  }
}

macro_rules! impl_into_vnode {
  { $( $T:ty ),*$( , )? } => {
    $(
      impl From<$T> for VNode {
        fn from(value: $T) -> Self {
          VNode(value.into())
        }
      }

      impl From<$T> for VNodeList {
        fn from(value: $T) -> Self {
          VNodeList::from(VNode::from(value))
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
#[derive(Debug, Default, Clone)]
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

impl From<VNodeList> for VNode {
  fn from(value: VNodeList) -> Self {
    if value.arr.length() == 1 {
      VNode(value.arr.get(0))
    } else {
      VNode(value.into())
    }
  }
}

impl From<VNode> for VNodeList {
  fn from(value: VNode) -> Self {
    c![value]
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

impl From<()> for VNodeList {
  fn from(_: ()) -> Self {
    VNodeList::new()
  }
}

macro_rules! impl_into_vnodelist_for_tuples {
  { $( ($( $x:ident ),*) ),* $(,)? } => {
    $(
      impl<$( $x, )*> From<($( $x, )*)> for VNodeList
      where $( $x: Into<VNodeList>, )*
      {
        fn from(($( $x, )*): ($( $x, )*)) -> VNodeList {
          c![$( $x.into(), )*]
        }
      }
    )*
  };
}

impl_into_vnodelist_for_tuples! {
  (A),
  (A, B),
  (A, B, C),
  (A, B, C, D),
  (A, B, C, D, E),
  (A, B, C, D, E, F),
  (A, B, C, D, E, F, G),
  (A, B, C, D, E, F, G, H),
  (A, B, C, D, E, F, G, H, I),
  (A, B, C, D, E, F, G, H, I, J),
  (A, B, C, D, E, F, G, H, I, J, K),
  (A, B, C, D, E, F, G, H, I, J, K, L),
  (A, B, C, D, E, F, G, H, I, J, K, L, M),
  (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
  (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O),
  (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P),
}
