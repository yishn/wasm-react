use crate::{
  children, classnames, deps, h, hooks, props::Style, Callable, Component,
  VNode, VNodeList,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct AppState {
  pub counter: i32,
  pub logs: Vec<&'static str>,
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
    let state = hooks::use_state(|| AppState {
      counter: 11,
      logs: vec![],
    });
    let warning = state.counter >= 50;

    hooks::use_effect(
      {
        let state = state.clone();
        move || {
          state.update(move |state| {
            state.logs.push(if warning {
              "Counter is now above 50 🎉"
            } else {
              "Counter is now below 50"
            })
          });

          || ()
        }
      },
      deps![warning],
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
        },
        //
        h!(ul.["logs"]).build(children![
          h!(li).build(children!["Started..."]),
          state
            .logs
            .iter()
            .map(|&log| h!(li).build(children![log]))
            .collect::<VNodeList>()
        ])
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
