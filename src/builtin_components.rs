use crate::{props::Props, react_bindings, VNode, VNodeList};

/// Can be used to create a [React fragment][fragment].
///
/// [fragment]: https://reactjs.org/docs/fragments.html
///
/// # Example
///
/// ```
/// Fragment.build(children![
///   h!(h1).build(children!["Hello World!"]),
///   h!(div).build(children!["No wrapper element"]),
/// ])
/// ```
pub struct Fragment;

impl Fragment {
  /// Returns a [`VNode`] which represents a [React fragment][fragment].
  ///
  /// [fragment]: https://reactjs.org/docs/fragments.html
  pub fn build(&self, children: VNodeList) -> VNode {
    VNode(react_bindings::create_builtin_component(
      "Fragment",
      &Props::new().into(),
      &children.into(),
    ))
  }
}
