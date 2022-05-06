use super::Props;
use crate::{create_element, Callback, VNode, VNodeList};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

/// The builder that powers [`h!`].
pub struct H<'a> {
  pub(crate) tag: &'a str,
  pub(crate) props: Props,
}

impl<'a> H<'a> {
  /// Creates a new instance of [`H`]. It is recommended to use the [`h!`]
  /// macro instead.
  pub fn new(tag: &'a str) -> Self {
    Self {
      tag,
      props: Props::new(),
    }
  }

  /// Sets an attribute on the [`VNode`].
  pub fn attr(mut self, key: &str, value: impl Into<JsValue>) -> Self {
    self.props = self.props.insert(key, value);
    self
  }

  /// Sets a callback value to an attribute on the [`VNode`].
  pub fn attr_callback<T, U>(
    mut self,
    key: &str,
    f: Callback<T, U>,
  ) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    self.props = self.props.insert_callback(key, f);
    self
  }

  /// Builds the [`VNode`] and returns it with the given children. Use
  /// [`children!`] for easier construction of the children.
  pub fn build(self, children: VNodeList) -> VNode {
    create_element(self.tag, self.props, children)
  }
}

/// This macro is to be used in conjunction with [`h!`]. It will take various
/// objects of [`Into<VNode>`](VNode) and builds a JS array.
///
/// # Example
///
/// ```
/// h!(div).build(children![
///   "Counter: ", 5,
///   SomeComponent {
///     some_prop,
///   },
///   h!(h1).build(children!["Hello World"]),
/// ])
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

/// A convenience macro to [`create_element()`] for creating HTML elements.
/// This macro returns a builder [`H`] which provides auto-completion for HTML
/// attributes and events.
///
/// # Example
///
/// ```
/// h!(div)
///   .attr("id", "app")
///   .build(children![
///     h!(h1).build(children!["Hello World!"])
///   ])
///
/// // <div id="app"><h1>Hello World!</h1></div>
/// ```
///
/// It is also possible to add classes to the element using an array notation.
/// You can use the same syntax as [`classnames!`](crate::classnames).
///
/// ```
/// h!(div.["some-class", "warning"])
///   .build(children!["This is a warning!"])
///
/// // <div class="some-class warning">This is a warning!</div>
/// ```
#[macro_export]
macro_rules! h {
  ($tag:ident $( .[$( $tt:tt )*] )?) => {
    $crate::props::H::new(stringify!($tag))
      $( .class_name(classnames![$( $tt )*]) )?
  };
}
