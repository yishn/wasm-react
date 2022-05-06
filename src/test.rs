use crate::{
  children, classnames, deps, h,
  hooks::{self, use_callback},
  props::Style,
  Callable, Callback, Component, VNode, VNodeList, Void,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = Math)]
  pub fn random() -> f64;

  #[wasm_bindgen(js_namespace = console)]
  pub fn log(input: &str);
}

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

    let handle_increment = use_callback(
      {
        let mut state = state.clone();
        let diff = self.diff;

        move |_| state.update(move |state| state.counter += diff)
      },
      deps!(self.diff),
    );

    let handle_decrement = use_callback(
      {
        let mut state = state.clone();
        let diff = self.diff;

        move |_| state.update(move |state| state.counter -= diff)
      },
      deps!(self.diff),
    );

    // hooks::use_effect(
    //   {
    //     let state = state.clone();
    //     move || {
    //       state.update(move |state| {
    //         state.logs.push(if warning {
    //           "Counter is now above 50 🎉"
    //         } else {
    //           "Counter is now below 50"
    //         })
    //       });

    //       || ()
    //     }
    //   },
    //   deps!(warning),
    // );

    h!(div.["app-container", warning.then(|| "warning")])
      .attr("data-counter", state.counter)
      .build(children![
        h!(h2)
          .style(Style::new().color(warning.then(|| "red")))
          .build(children!["Counter: ", state.counter]),
        //
        Counter {
          counter: state.counter,
          on_increment: Some(handle_increment.clone()),
          on_decrement: Some(handle_decrement.clone()),
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
pub struct Counter {
  pub counter: i32,
  pub on_increment: Option<Callback<Void>>,
  pub on_decrement: Option<Callback<Void>>,
}

impl Component for Counter {
  fn name() -> &'static str {
    "Counter"
  }

  fn render(&self) -> VNode {
    let handle_decrement = use_callback(
      {
        let on_decrement = self.on_decrement.clone();
        move |_| on_decrement.call(Void)
      },
      deps!(self.on_decrement.clone()),
    );

    let handle_increment = use_callback(
      {
        let on_increment = self.on_increment.clone();
        move |_| on_increment.call(Void)
      },
      deps!(self.on_increment.clone()),
    );

    h!(div.["counter-component"]).build(children![
      h!(button)
        .on_click(&handle_decrement)
        .build(children!["Decrement"]),
      " ",
      h!(button.["default"])
        .on_click(&handle_increment)
        .html_type("submit")
        .build(children!["Increment"])
    ])
  }
}
