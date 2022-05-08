use crate::{create_element, props::Props, react_bindings, VNode, VNodeList};

/// Can be used to create a [React fragment][fragment].
///
/// [fragment]: https://reactjs.org/docs/fragments.html
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// # fn f() -> VNode {
/// Fragment.build(children![
///   h!(h1).build(children!["Hello World!"]),
///   h!(div).build(children!["No wrapper element"]),
/// ])
/// # }
/// ```
pub struct Fragment;

impl Fragment {
  /// Returns a [`VNode`] which represents a [React fragment][fragment].
  ///
  /// [fragment]: https://reactjs.org/docs/fragments.html
  pub fn build(&self, children: VNodeList) -> VNode {
    create_element(&react_bindings::FRAGMENT, Props::new().as_ref(), children)
  }
}
