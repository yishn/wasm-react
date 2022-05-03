mod callback;
mod component;
mod react;
mod vnode;

use js_sys::{Object, Reflect};
use props::Style;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

pub mod hooks;
pub mod props;
pub use callback::*;
pub use component::*;
pub use vnode::*;

pub fn create_element<'a>(
  tag: JsValue,
  props: impl IntoIterator<Item = (&'a str, JsValue)>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  let props_obj = Object::new();

  for (prop, value) in props.into_iter() {
    Reflect::set(&props_obj, &(*prop).into(), &value).unwrap();
  }

  VNode(react::create_element(
    tag,
    props_obj.into(),
    children.into_iter().map(|c| JsValue::from(c)).collect(),
  ))
}

pub fn html<'a>(
  tag: &str,
  props: impl IntoIterator<Item = (&'a str, JsValue)>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  create_element(tag.into(), props, children)
}

#[doc(hidden)]
#[derive(Debug)]
pub struct AppState {
  pub counter: i32,
  pub diff: i32,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      counter: 11,
      diff: 5,
    }
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(input: &str);
}

#[doc(hidden)]
#[derive(Debug)]
pub struct App;

impl Component for App {
  fn js_name() -> &'static str
  where
    Self: Sized,
  {
    "App"
  }

  fn render(&self) -> VNode {
    let state = hooks::use_state(|| AppState::default());

    hooks::use_effect(
      {
        let state = state.clone();

        move || {
          log(if state.counter >= 50 {
            "Counter is above 50 🎉"
          } else {
            "Counter is below 50"
          });

          || ()
        }
      },
      Some(&[(state.counter >= 50).into()]),
    );

    html(
      "div",
      [props::classnames("app")],
      [Counter {
        counter: state.counter,
        on_increment: Some({
          let state = state.clone();

          Callback::new(move |_| {
            state.update(|state| state.counter += state.diff);
          })
        }),
        on_decrement: Some({
          let state = state.clone();

          Callback::new(move |_| {
            state.update(|state| state.counter -= state.diff);
          })
        }),
      }
      .into_vnode()],
    )
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = createApp)]
pub fn create_app() -> VNode {
  App.into_vnode()
}

#[doc(hidden)]
pub struct Counter {
  pub counter: i32,
  pub on_increment: Option<Callback<()>>,
  pub on_decrement: Option<Callback<()>>,
}

impl Component for Counter {
  fn js_name() -> &'static str
  where
    Self: Sized,
  {
    "Counter"
  }

  fn render(&self) -> VNode {
    html(
      "div",
      [props::classnames("counter")],
      [
        html(
          "h2",
          [Style::new()
            .add(
              "color",
              if self.counter >= 50 {
                Some("red")
              } else {
                None
              },
            )
            .into()],
          ["Counter: ".into(), self.counter.into()],
        ),
        html(
          "button",
          [props::on_click({
            let on_decrement = self.on_decrement.clone();
            move |_| {
              if let Some(on_decrement) = on_decrement.as_ref() {
                on_decrement(());
              }
            }
          })],
          ["Decrement".into()],
        ),
        " ".into(),
        html(
          "button",
          [props::on_click({
            let on_increment = self.on_increment.clone();
            move |_| {
              if let Some(on_increment) = on_increment.as_ref() {
                on_increment(());
              }
            }
          })],
          ["Increment".into()],
        ),
      ],
    )
  }
}
