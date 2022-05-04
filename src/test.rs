use crate::{
  hooks::{self, Deps},
  html,
  props::{Attr, Style},
  Callable, Component, VNode,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(input: &str);
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct AppState {
  pub counter: i32,
}

impl Default for AppState {
  fn default() -> Self {
    Self { counter: 11 }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct App {
  diff: i32,
}

impl Component for App {
  fn name() -> &'static str {
    "App"
  }

  fn render(&self) -> VNode {
    let state = hooks::use_state(|| AppState::default());

    hooks::use_effect(
      {
        let state = state.clone();

        move || {
          log(if state.counter >= 50 {
            "Counter is now above 50 🎉"
          } else {
            "Counter is now below 50"
          });

          || ()
        }
      },
      Deps::None.push(state.counter >= 50),
    );

    html(
      "div",
      Attr::new()
        .class_name("app")
        .insert("data-counter", state.counter),
      [Counter {
        counter: state.counter,
        on_increment: Some({
          let state = state.clone();
          let diff = self.diff;
          move |_| state.update(move |state| state.counter += diff)
        }),
        on_decrement: Some({
          let state = state.clone();
          let diff = self.diff;
          move |_| state.update(move |state| state.counter -= diff)
        }),
      }
      .into_vnode()],
    )
  }
}

#[doc(hidden)]
#[allow(dead_code)]
#[wasm_bindgen(js_name = createApp)]
pub fn create_app() -> JsValue {
  App { diff: 5 }.into_vnode().into()
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Counter<F, G>
where
  F: Fn(()) + Clone + 'static,
  G: Fn(()) + Clone + 'static,
{
  pub counter: i32,
  pub on_increment: Option<F>,
  pub on_decrement: Option<G>,
}

impl<F, G> Component for Counter<F, G>
where
  F: Fn(()) + Clone + 'static,
  G: Fn(()) + Clone + 'static,
{
  fn name() -> &'static str {
    "Counter"
  }

  fn render(&self) -> VNode {
    html(
      "div",
      Attr::new().class_name("counter"),
      [
        html(
          "h2",
          Attr::new().style(Style::new().insert(
            "color",
            if self.counter >= 50 {
              Some("red")
            } else {
              None
            },
          )),
          ["Counter: ".into(), self.counter.into()],
        ),
        html(
          "button",
          Attr::new().on_click({
            let on_decrement = self.on_decrement.clone();
            move |_| on_decrement.call(())
          }),
          ["Decrement".into()],
        ),
        " ".into(),
        html(
          "button",
          Attr::new().on_click({
            let on_increment = self.on_increment.clone();
            move |_| on_increment.call(())
          }),
          ["Increment".into()],
        ),
      ],
    )
  }
}
