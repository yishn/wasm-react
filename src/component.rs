use crate::VNode;
use js_sys::Object;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub trait Component: 'static {
  fn render(&self) -> VNode;

  fn extra_props(&self) -> JsValue {
    Object::new().into()
  }
}

#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
struct ComponentWrapper(Box<dyn Component>);

impl ComponentWrapper {
  pub fn from(component: impl Component) -> ComponentWrapper {
    ComponentWrapper(Box::new(component))
  }
}

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  pub fn render(&self) -> JsValue {
    self.0.render().into()
  }
}
