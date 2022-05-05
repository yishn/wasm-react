use crate::{
  children, h,
  hooks::{self, Deps},
  props::Style,
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

    h("div")
      .class_name("app")
      .attr("data-counter", state.counter)
      .build_with(children![Counter {
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
      .into_vnode()])
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
  fn render(&self) -> VNode {
    h("div").class_name("counter").build_with(children![
      h("h2")
        .style(Style::new().color(if self.counter >= 50 {
          Some("red")
        } else {
          None
        }))
        .build_with(children!["Counter: ", self.counter]),
      h("button")
        .on_click({
          let on_decrement = self.on_decrement.clone();
          move |_| on_decrement.call(())
        })
        .build_with(children!["Decrement"]),
      " ",
      h("button")
        .on_click({
          let on_increment = self.on_increment.clone();
          move |_| on_increment.call(())
        })
        .build_with(children!["Increment"])
    ])
  }
}
