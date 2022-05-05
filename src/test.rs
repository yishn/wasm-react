use crate::{
  children, classnames, h,
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

#[derive(Debug, Clone)]
pub struct AppState {
  pub counter: i32,
}

impl Default for AppState {
  fn default() -> Self {
    Self { counter: 11 }
  }
}

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
    let warning = state.counter >= 50;

    hooks::use_effect(
      move || {
        log(if warning {
          "Counter is now above 50 🎉"
        } else {
          "Counter is now below 50"
        });

        || ()
      },
      Deps::None.push(warning),
    );

    h!(div.["app-container"])
      .attr("data-counter", state.counter)
      .build(children![
        h!(h2)
          .style(Style::new().color(if warning { Some("red") } else { None }))
          .build(children!["Counter: ", state.counter]),
        //
        Counter {
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
          })
        }
      ])
  }
}

#[allow(dead_code)]
#[wasm_bindgen(js_name = createApp)]
pub fn create_app() -> JsValue {
  VNode::from(App { diff: 5 }).into()
}

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
    h!(div.["counter-component"]).build(children![h!(form)
      .on_submit(|evt| evt.prevent_default())
      .build(children![
        h!(button)
          .on_click({
            let on_decrement = self.on_decrement.clone();
            move |_| on_decrement.call(())
          })
          .build(children!["Decrement"]),
        " ",
        h!(button.["default"])
          .on_click({
            let on_increment = self.on_increment.clone();
            move |_| on_increment.call(())
          })
          .typ("submit")
          .build(children!["Increment"])
      ])])
  }
}
