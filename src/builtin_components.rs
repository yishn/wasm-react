use crate::{
  props::{HType, H},
  react_bindings, VNodeList,
};
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
  fn with_js<T>(&self, f: impl FnOnce(&JsValue) -> T) -> T {
    f(&react_bindings::FRAGMENT)
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
  fn with_js<T>(&self, f: impl FnOnce(&JsValue) -> T) -> T {
    f(&react_bindings::SUSPENSE)
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
