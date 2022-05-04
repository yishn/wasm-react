use crate::{react_bindings, VNode};
use wasm_bindgen::prelude::*;

pub trait Component {
  fn name() -> &'static str
  where
    Self: Sized;

  fn render(&self) -> VNode;

  fn into_vnode(self) -> VNode
  where
    Self: Sized + 'static,
  {
    VNode(react_bindings::create_component(
      Self::name(),
      ComponentWrapper(Box::new(self)).into(),
    ))
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(Box<dyn Component>);

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen]
  pub fn render(props: &ComponentWrapper) -> JsValue {
    props.0.render().into()
  }
}
