use super::{Component, MemoComponentWrapper};
use crate::{
  react_bindings::{self},
  VNode,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Memo<C>(pub(super) C);

impl<C> Component for Memo<C>
where
  C: Component + PartialEq,
{
  fn render(self, _children: VNode) -> impl Into<VNode> {
    ()
  }

  fn build(self, props: &js_sys::Object) -> VNode {
    let original_name = C::name();
    let memo_name = Self::name();
    let component = MemoComponentWrapper::from(self.0);

    VNode(react_bindings::create_rust_memo_component(
      original_name,
      memo_name,
      component,
      props,
    ))
  }
}
