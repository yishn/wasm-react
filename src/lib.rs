mod callback;
mod react;
mod vnode;

use js_sys::{Object, Reflect};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

pub mod hooks;
pub mod props;
pub use callback::*;
pub use vnode::*;

pub fn create_element(
  tag: JsValue,
  props: impl IntoIterator<Item = (&'static str, JsValue)>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  let props_obj = Object::new();

  for (prop, value) in props.into_iter() {
    Reflect::set(&props_obj, &(*prop).into(), &value).ok();
  }

  VNode(react::create_element(
    tag,
    props_obj.into(),
    children.into_iter().map(|c| JsValue::from(c)).collect(),
  ))
}

pub fn html(
  tag: &'static str,
  props: impl IntoIterator<Item = (&'static str, JsValue)>,
  children: impl IntoIterator<Item = VNode>,
) -> VNode {
  create_element(tag.into(), props, children)
}

pub fn render_component<P: Into<JsValue>>(
  name: &'static str,
  props: P,
) -> VNode {
  VNode(react::render_component(name, props.into()))
}

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
pub struct App;

#[wasm_bindgen]
impl App {
  #[wasm_bindgen]
  pub fn render() -> VNode {
    let state = hooks::use_state(|| AppState::default());

    html(
      "div",
      [props::classnames("app")],
      [Counter {
        counter: state.counter,
        on_increment: {
          let state = state.clone();

          Callback::new(move |_| {
            state.update(|state| state.counter += state.diff);
          })
        },
        on_decrement: {
          let state = state.clone();

          Callback::new(move |_| {
            state.update(|state| state.counter -= state.diff);
          })
        },
      }
      .into()],
    )
  }
}

#[wasm_bindgen]
pub struct Counter {
  counter: i32,
  on_increment: Callback<()>,
  on_decrement: Callback<()>,
}

impl From<Counter> for VNode {
  fn from(value: Counter) -> Self {
    render_component("Counter", value)
  }
}

#[wasm_bindgen]
impl Counter {
  #[wasm_bindgen]
  pub fn render(props: Counter) -> VNode {
    html(
      "div",
      [props::classnames("counter")],
      [
        html("h2", None, ["Counter: ".into(), props.counter.into()]),
        html(
          "button",
          [("onClick", {
            let on_decrement = props.on_decrement.clone();

            Callback::new(move |_: JsValue| on_decrement(())).into()
          })],
          ["Decrement".into()],
        ),
        " ".into(),
        html(
          "button",
          [("onClick", {
            let on_increment = props.on_increment.clone();

            Callback::new(move |_: JsValue| on_increment(())).into()
          })],
          ["Increment".into()],
        ),
      ],
    )
  }
}
