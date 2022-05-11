use crate::{create_element, props::Props, react_bindings, VNode, VNodeList};

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
/// Fragment.build(c![
///   h!(h1).build(c!["Hello World!"]),
///   h!(div).build(c!["No wrapper element"]),
/// ])
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Fragment;

impl Fragment {
  /// Builds a [`VNode`] of the component.
  pub fn build(&self, children: VNodeList) -> VNode {
    create_element(&react_bindings::FRAGMENT, Props::new(), children)
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
/// #   fn name() -> &'static str { "" }
/// #   fn render(&self) -> VNode { VNode::default() }
/// # }
/// #
/// # fn f() -> VNode {
/// Suspense::new()
///   .fallback(c![
///     h!(div[."loading"]).build(c!["Loading..."]),
///   ])
///   .build(c![
///     SomeLazyComponent { /* ... */ }
///   ])
/// # }
/// ```
#[derive(Default, Clone)]
pub struct Suspense {
  fallback: VNodeList,
}

impl Suspense {
  /// Creates a new `React.Suspense` component builder.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets a fallback when loading lazy descendant components.
  pub fn fallback(mut self, children: VNodeList) -> Self {
    self.fallback = children;
    self
  }

  /// Builds the [`VNode`] of the component.
  pub fn build(&self, children: VNodeList) -> VNode {
    create_element(
      &react_bindings::SUSPENSE,
      Props::new().insert("fallback", self.fallback.as_ref()),
      children,
    )
  }
}
