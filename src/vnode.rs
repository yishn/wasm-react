use crate::{props::Props, react_bindings, Component, ComponentWrapper};
use js_sys::Array;
use wasm_bindgen::JsValue;

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
      T::name(),
      &Props::new()
        .insert("key", &value.key().into())
        .insert("component", &ComponentWrapper(Box::new(value)).into())
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
///
/// Use the [`children!`] macro to build a [`VNodeList`] more conveniently. You
/// can also collect an iterator of [`VNode`] into a [`VNodeList`]:
///
/// ```
/// # use wasm_react::*;
/// #
/// # fn f() -> VNodeList {
/// vec!["first item", "second item", "third item"]
///   .iter()
///   .map(|&x| h!(li).build(children![x]))
///   .collect::<VNodeList>()
/// # }
/// ```
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

impl AsRef<JsValue> for VNodeList {
  fn as_ref(&self) -> &JsValue {
    &self.0
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

/// This macro will take various objects of [`Into<VNode>`](VNode) and builds a
/// [`VNodeList`].
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// #
/// # struct SomeComponent { some_prop: () }
/// # impl Component for SomeComponent {
/// #   fn name() -> &'static str { "" }
/// #   fn render(&self) -> VNode { VNode::empty() }
/// # }
/// #
/// # fn f(some_prop: ()) -> VNode {
/// h!(div).build(children![
///   "Counter: ", 5,
///   SomeComponent {
///     some_prop,
///   },
///   h!(h1).build(children!["Hello World"]),
/// ])
/// # }
/// ```
#[macro_export]
macro_rules! children {
  [$( $into_vnode:expr ),* $(,)?] => {
    {
      let arr = $crate::VNodeList::new();
      $( arr.push($into_vnode.into()); )*
      arr
    }
  };
}
