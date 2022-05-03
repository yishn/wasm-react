use crate::{react, VNode};
use wasm_bindgen::prelude::*;

pub trait Component {
  fn js_name() -> &'static str
  where
    Self: Sized;

  fn render(&self) -> VNode;

  fn into_vnode(self) -> VNode
  where
    Self: Sized + 'static,
  {
    VNode(react::create_component(
      Self::js_name(),
      JsComponentWrapper(Box::new(self)).into(),
    ))
  }
}

#[wasm_bindgen(js_name = __JsComponentWrapper)]
pub struct JsComponentWrapper(Box<dyn Component>);

#[wasm_bindgen(js_class = __JsComponentWrapper)]
impl JsComponentWrapper {
  #[wasm_bindgen]
  pub fn render(props: &JsComponentWrapper) -> VNode {
    props.0.render()
  }
}
