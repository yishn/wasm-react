use wasm_bindgen::JsValue;
use wasm_react::{export_components, h, Component, VNode};

struct App;

impl Component for App {
  fn render(&self, _children: VNode) -> impl Into<VNode> {
    (
      h!(h1."title").children((
        "Hello ", //
        h!(em).children("World"),
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
    h!(div."counter").children((
      h!(span).children(("Counter: ", self.count)),
      " ",
      h!(button).children("Decrement"),
      " ",
      h!(button).children("Increment"),
    ))
  }
}
