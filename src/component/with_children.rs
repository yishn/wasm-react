use super::Component;
use crate::{Prop, VNode};
use js_sys::{JsString, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

#[derive(Debug, Clone, Copy)]
pub struct WithChildren<C> {
  pub(super) component: C,
  pub(super) children: Prop<VNode>,
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHILDREN: JsString = "children";
}

impl<C> Component for WithChildren<C>
where
  C: Component,
{
  fn render(self, children: VNode) -> impl Into<VNode> {
    self.component.render(children)
  }

  fn name() -> &'static str {
    C::name()
  }

  fn props(self) -> Object {
    let props = self.component.props();

    CHILDREN.with(|children| {
      Reflect::set(&props, &children, &self.children.get()).unwrap_throw()
    });

    props
  }

  fn build(self, props: &Object) -> VNode {
    self.component.build(props)
  }
}
