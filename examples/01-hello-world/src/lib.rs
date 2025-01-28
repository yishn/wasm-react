use wasm_bindgen::JsValue;
use wasm_react::{Component, Tag, VNode};

pub struct App;

impl Component for App {
  fn render(&self, _children: VNode) -> VNode {
    Tag::new("h1")
      .classname("title")
      .children((
        "Hello ", //
        Tag::new("em").children("World").build(),
        "!",
      ))
      .build()
  }
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_value: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}
