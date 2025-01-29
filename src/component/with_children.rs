use super::Component;
use crate::VNode;
use js_sys::{JsString, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

#[derive(Debug, Default, Clone)]
pub struct WithChildren<C: Component> {
  pub(super) component: C,
  pub(super) children: VNode,
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHILDREN: JsString = "children";
}

impl<C: Component> Component for WithChildren<C> {
  fn render(&self, children: VNode) -> impl Into<VNode> {
    self.component.render(children)
  }

  fn name() -> &'static str {
    C::name()
  }

  fn props(&self) -> Object {
    let props = self.component.props();

    CHILDREN.with(|children| {
      Reflect::set(&props, &children, &self.children).unwrap_throw()
    });

    props
  }

  fn build(self, props: &Object) -> VNode {
    self.component.build(props)
  }
}
