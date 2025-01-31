use wasm_bindgen::JsValue;
use wasm_react::{
  export_components, h, hooks::use_state, Callback, Component, Prop, VNode,
  Void,
};

#[derive(Debug, Clone, Copy)]
struct App;

impl Component for App {
  fn render(self, _children: VNode) -> impl Into<VNode> {
    let (count, count_mut) = use_state(|| 0);

    (
      h!(h1."title").children((
        "Hello ", //
        h!(em).children("World"),
        "!",
      )),
      Counter {
        count: *count.get(),
        on_increment: Callback::new(move |_| {
          count_mut.update(|count| count + 1)
        })
        .into(),
        on_decrement: Callback::new(move |_| {
          count_mut.update(|count| count - 1)
        })
        .into(),
      },
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

#[derive(Debug, Clone, Copy)]
struct Counter {
  count: i32,
  on_increment: Prop<Callback<Void>>,
  on_decrement: Prop<Callback<Void>>,
}

impl Component for Counter {
  fn render(self, _children: VNode) -> impl Into<VNode> {
    h!(div."counter").children((
      h!(button)
        .on_click(move |_| self.on_decrement.call(Void))
        .children("Decrement"),
      " ",
      h!(span).children(("Counter: ", self.count)),
      " ",
      h!(button)
        .on_click(move |_| self.on_increment.call(Void))
        .children("Increment"),
    ))
  }
}
