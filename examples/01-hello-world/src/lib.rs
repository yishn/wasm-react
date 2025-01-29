use wasm_bindgen::JsValue;
use wasm_react::{
  export_components, h, hooks::use_state, Callback, Component, VNode, Void,
};

struct App;

impl Component for App {
  fn render(&self, _children: VNode) -> impl Into<VNode> {
    let (counter, counter_mut) = use_state(|| 0);

    (
      h!(h1."title").children((
        "Hello ", //
        h!(em).children("World"),
        "!",
      )),
      Counter {
        count: *counter.get(),
        on_increment: Callback::new(move |_| {
          counter_mut.update(|counter| counter + 1)
        }),
        on_decrement: Callback::new(move |_| {
          counter_mut.update(|counter| counter - 1)
        }),
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

struct Counter {
  count: i32,
  on_increment: Callback<Void>,
  on_decrement: Callback<Void>,
}

impl Component for Counter {
  fn render(&self, _children: VNode) -> impl Into<VNode> {
    let on_decrement = self.on_decrement;
    let on_increment = self.on_increment;

    h!(div."counter").children((
      h!(span).children(("Counter: ", self.count)),
      " ",
      h!(button)
        .on_click(move |_| on_decrement.call(Void))
        .children("Decrement"),
      " ",
      h!(button)
        .on_click(move |_| on_increment.call(Void))
        .children("Increment"),
    ))
  }
}
