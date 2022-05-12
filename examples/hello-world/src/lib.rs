use wasm_bindgen::JsValue;
use wasm_react::{c, export_component, h, Component, VNode};

pub struct App;

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

impl Component for App {
  fn render(&self) -> VNode {
    h!(h1).build(c!["Hello World!"])
  }
}

export_component!(App);
