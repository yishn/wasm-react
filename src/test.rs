use crate::{
  c,
  callback::{Callable, Callback, Void},
  create_context, create_element, export_component, h,
  hooks::{use_callback, use_context, use_effect, use_js_ref, use_state, Deps},
  props::{Props, Style},
  Component, Context, ContextProvider, VNode,
};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = Math)]
  pub fn random() -> f64;

  #[wasm_bindgen(js_namespace = console)]
  pub fn log(input: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  pub fn log_js(input: &JsValue);
}

thread_local! {
  static THEME_CONTEXT: Context<Theme> = create_context(Theme::LightMode);
}

pub enum Theme {
  DarkMode,
  LightMode,
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

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    let diff = Reflect::get(&value, &"diff".into())?
      .as_f64()
      .ok_or(JsError::new("`diff` property not found"))?;

    Ok(App { diff: diff as i32 })
  }
}

export_component!(App);

impl Component for App {
  fn render(&self) -> VNode {
    let state = use_state(|| AppState {
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
      Deps::some(self.diff),
    );

    let handle_decrement = use_callback(
      {
        let mut state = state.clone();
        let diff = self.diff;

        move |_| state.update(move |state| state.counter -= diff)
      },
      Deps::some(self.diff),
    );

    use_effect(
      {
        let mut state = state.clone();

        move || {
          state.update(|state| {
            state.logs.push(if warning {
              "Counter is now above 50 🎉"
            } else {
              "Counter is now below 50"
            })
          });

          || ()
        }
      },
      Deps::some(warning),
    );

    ContextProvider::from(&THEME_CONTEXT)
      .value(Theme::DarkMode)
      .build(c![
        //
        h!(div[#"app-container".warning])
          .attr("data-counter", &state.counter.into())
          .build(c![
            create_element(
              &WELCOME,
              Props::new().insert("welcome", &"Welcome!".into()),
              c![],
            ),
            h!(h2)
              .style(Style::new().color(warning.then(|| "red")))
              .build(c!["Counter: ", state.counter]),
            //
            Counter {
              counter: state.counter,
              on_increment: Some(handle_increment.into()),
              on_decrement: Some(handle_decrement.into()),
            },
            //
            h!(ul[."logs"]).build(c![
              h!(li).build(c!["Started..."]),
              ..state.logs.iter().map(|&log| h!(li).build(c![log]))
            ])
          ])
      ])
  }
}

#[derive(Debug, Clone)]
pub struct Counter {
  pub counter: i32,
  pub on_increment: Option<Callback<Void>>,
  pub on_decrement: Option<Callback<Void>>,
}

impl Component for Counter {
  fn render(&self) -> VNode {
    let dark_mode = match *use_context(&THEME_CONTEXT) {
      Theme::DarkMode => true,
      Theme::LightMode => false,
    };
    let element_ref = use_js_ref(None::<Element>);
    let handle_decrement = use_callback(
      {
        let on_decrement = self.on_decrement.clone();
        move |_| on_decrement.call(Void)
      },
      Deps::some(self.on_decrement.clone()),
    );

    let handle_increment = use_callback(
      {
        let on_increment = self.on_increment.clone();
        move |_| on_increment.call(Void)
      },
      Deps::some(self.on_increment.clone()),
    );

    use_effect(
      {
        let element_ref = element_ref.clone();

        move || {
          log_js(
            &element_ref
              .current()
              .map(|x| x.into())
              .unwrap_or(JsValue::undefined()),
          );

          || ()
        }
      },
      Deps::some(element_ref.current_untyped()),
    );

    h!(div[."counter-component".{dark_mode.then(|| "dark")}])
      .ref_container(&element_ref)
      .build(c![
        h!(button)
          .on_click(&handle_decrement)
          .build(c!["Decrement"]),
        " ",
        h!(button[."default"])
          .on_click(&handle_increment)
          .html_type("submit")
          .build(c!["Increment"])
      ])
  }
}

#[wasm_bindgen(inline_js = "
  import React from 'https://cdn.skypack.dev/react';

  export function Welcome(props) {
    return React.createElement('h1', {}, props.welcome);
  }
")]
extern "C" {
  #[wasm_bindgen(js_name = Welcome)]
  static WELCOME: JsValue;
}
