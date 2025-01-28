use super::{Component, ComponentWithChildren};
use crate::VNode;
use js_sys::{JsString, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

#[derive(Debug, Default, Clone)]
pub struct WithChildren<C: ComponentWithChildren> {
  pub(super) component: C,
  pub(super) children: VNode,
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHILDREN: JsString = "children";
}

impl<C: ComponentWithChildren> Component for WithChildren<C> {
  fn render(&self) -> VNode {
    self.component.render()
  }

  fn extra_props(&self) -> Object {
    let props = self.component.extra_props();

    CHILDREN.with(|children| {
      Reflect::set(&props, &children, &self.children).unwrap_throw()
    });

    props
  }
}
