use wasm_bindgen::JsValue;
use wasm_react::{export_components, Component, Tag, VNode};

struct App;

impl Component for App {
  fn render(&self, _children: VNode) -> impl Into<VNode> {
    (
      Tag::new("h1").classname("title").children((
        "Hello ",
        Tag::new("em").children("World"),
        "!",
      )),
      Counter { count: 0 },
    )
  }
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_value: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

export_components! {
  App,
}

struct Counter {
  count: i32,
}

impl Component for Counter {
  fn render(&self, _children: VNode) -> impl Into<VNode> {
    Tag::new("div").classname("counter").children((
      Tag::new("span").children(("Counter: ", self.count)),
      " ",
      Tag::new("button").children("Decrement"),
      " ",
      Tag::new("button").children("Increment"),
    ))
  }
}
