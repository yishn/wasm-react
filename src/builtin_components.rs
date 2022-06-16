use crate::{
  props::{HType, H},
  react_bindings, Component, MemoComponentWrapper, VNode, VNodeList,
};
use std::{any::type_name, borrow::Cow};
use wasm_bindgen::JsValue;

/// Can be used to create a [React fragment][fragment].
///
/// [fragment]: https://reactjs.org/docs/fragments.html
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// #
/// # fn f() -> VNode {
/// Fragment::new().build(c![
///   h!(h1).build(c!["Hello World!"]),
///   h!(div).build(c!["No wrapper element"]),
/// ])
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Fragment;

impl HType for Fragment {
  fn as_js(&self) -> Cow<'_, JsValue> {
    Cow::Borrowed(&react_bindings::FRAGMENT)
  }
}

impl Fragment {
  /// Creates a new `React.Fragment` component builder.
  pub fn new() -> H<Fragment> {
    H::new(Fragment)
  }
}

/// A component that specifies the loading indicator when loading lazy descendant
/// components.
///
/// For more information, see [React documentation about code-splitting][docs].
///
/// [docs]: https://reactjs.org/docs/code-splitting.html
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// #
/// # struct SomeLazyComponent {}
/// # impl Component for SomeLazyComponent {
/// #   fn render(&self) -> VNode { VNode::default() }
/// # }
/// #
/// # fn f() -> VNode {
/// Suspense::new()
///   .fallback(c![
///     h!(div[."loading"]).build(c!["Loading…"]),
///   ])
///   .build(c![
///     SomeLazyComponent { /* … */ }.build()
///   ])
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Suspense;

impl HType for Suspense {
  fn as_js(&self) -> Cow<'_, JsValue> {
    Cow::Borrowed(&react_bindings::SUSPENSE)
  }
}

impl Suspense {
  /// Creates a new `React.Suspense` component builder.
  pub fn new() -> H<Suspense> {
    H::new(Suspense)
  }
}

impl H<Suspense> {
  /// Sets a fallback when loading lazy descendant components.
  pub fn fallback(self, children: VNodeList) -> Self {
    self.attr("fallback", children.as_ref())
  }
}

/// Wraps your component to let React skip rendering it if props haven't changed.
///
/// If your component renders the same result given the same props, you can
/// memoize your component with [`Memo`] for a performance boost.
///
/// You have to implement [`PartialEq`] on your [`Component`] for this to work.
///
/// # Example
///
/// ```
/// # use std::rc::Rc;
/// # use wasm_react::*;
/// #[derive(PartialEq)]
/// struct MessageBox {
///   message: Rc<str>,
/// }
///
/// impl Component for MessageBox {
///   fn render(&self) -> VNode {
///     h!(h1[."message-box"]).build(c![&*self.message])
///   }
/// }
///
/// struct App;
///
/// impl Component for App {
///   fn render(&self) -> VNode {
///     h!(div[#"app"]).build(c![
///       Memo(MessageBox {
///         message: Rc::from("Hello World!"),
///       })
///       .build()
///     ])
///   }
/// }
/// ```
pub struct Memo<T: Component + PartialEq>(pub T);

impl<T: Component + PartialEq> Memo<T> {
  /// Returns a [`VNode`] to be included in a render function.
  pub fn build(self) -> VNode {
    self.build_with_key(None)
  }

  /// Returns a [`VNode`] to be included in a render function with the given
  /// [React key].
  ///
  /// [React key]: https://reactjs.org/docs/lists-and-keys.html
  pub fn build_with_key(self, key: Option<&str>) -> VNode {
    VNode(react_bindings::create_rust_memo_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<T>(),
      key,
      MemoComponentWrapper(Box::new(self.0)),
    ))
  }
}
