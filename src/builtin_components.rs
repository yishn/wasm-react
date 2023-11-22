use crate::{
  props::{HType, H},
  react_bindings, VNode,
};
use std::borrow::Cow;
use wasm_bindgen::JsValue;

/// A component that specifies the loading indicator when loading lazy descendant
/// components.
///
/// For more information, see [React documentation about code-splitting][docs].
///
/// [docs]: https://react.dev/reference/react/Suspense
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
///   .fallback(
///     h!(div[."loading"]).build("Loading…"),
///   )
///   .build(
///     SomeLazyComponent { /* … */ }.build()
///   )
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
  pub fn fallback(self, children: impl Into<VNode>) -> Self {
    self.attr("fallback", children.into().as_ref())
  }
}
