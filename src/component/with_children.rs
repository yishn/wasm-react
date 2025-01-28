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
  fn render(&self, children: VNode) -> VNode {
    self.component.render(children)
  }

  fn extra_props(&self) -> Object {
    let props = self.component.extra_props();

    CHILDREN.with(|children| {
      Reflect::set(&props, &children, &self.children).unwrap_throw()
    });

    props
  }
}
