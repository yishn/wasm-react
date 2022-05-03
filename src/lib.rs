mod callback;
mod component;
mod react;
mod vnode;

use attr::{Attr, Style};
use js_sys::{Object, Reflect};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

pub mod attr;
pub mod hooks;
pub use callback::*;
pub use component::*;
pub use vnode::*;

pub fn create_element<'a>(
  typ: JsValue,
  props: impl IntoIterator<Item = (&'a str, JsValue)>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  let props_obj = Object::new();

  for (prop, value) in props.into_iter() {
    Reflect::set(&props_obj, &(*prop).into(), &value).unwrap();
  }

  VNode(react::create_element(
    typ,
    props_obj.into(),
    children.into_iter().map(|c| JsValue::from(c)).collect(),
  ))
}

pub fn html(
  tag: &str,
  attr: Attr,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  VNode(react::create_element(
    tag.into(),
    attr.into(),
    children.into_iter().map(|c| JsValue::from(c)).collect(),
  ))
}

#[doc(hidden)]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct App;

impl Component for App {
  fn js_name() -> &'static str {
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
      Some(&[(state.counter >= 50).into()]),
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
          move || state.update(|state| state.counter += state.diff)
        }),
        on_decrement: Some({
          let state = state.clone();
          move || state.update(|state| state.counter -= state.diff)
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
#[derive(Debug, Clone)]
pub struct Counter<F, G>
where
  F: Fn() + Clone + 'static,
  G: Fn() + Clone + 'static,
{
  pub counter: i32,
  pub on_increment: Option<F>,
  pub on_decrement: Option<G>,
}

impl<F, G> Component for Counter<F, G>
where
  F: Fn() + Clone + 'static,
  G: Fn() + Clone + 'static,
{
  fn js_name() -> &'static str {
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
            move |_| on_decrement.as_ref().map(|f| f()).unwrap_or(())
          }),
          ["Decrement".into()],
        ),
        " ".into(),
        html(
          "button",
          Attr::new().on_click({
            let on_increment = self.on_increment.clone();
            move |_| on_increment.as_ref().map(|f| f()).unwrap_or(())
          }),
          ["Increment".into()],
        ),
      ],
    )
  }
}
