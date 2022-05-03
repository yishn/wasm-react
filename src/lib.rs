mod callback;
mod component;
mod react;
mod vnode;

use js_sys::{Object, Reflect};
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
    Reflect::set(&props_obj, &(*prop).into(), &value).ok();
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

#[doc(hidden)]
#[derive(Debug)]
pub struct App;

impl Component for App {
  fn render(_: Self) -> VNode {
    let state = hooks::use_state(|| AppState::default());

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

impl HasJsComponent for App {
  type JsComponent = JsApp;

  fn js_name() -> &'static str {
    "App"
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = App)]
pub struct JsApp {
  component: App,
}

impl From<JsApp> for App {
  fn from(value: JsApp) -> Self {
    value.component
  }
}

impl From<App> for JsApp {
  fn from(component: App) -> Self {
    Self { component }
  }
}

#[wasm_bindgen(js_class = App)]
impl JsApp {
  #[wasm_bindgen]
  pub fn render() -> VNode {
    App::render(App)
  }
}

#[doc(hidden)]
pub struct Counter {
  pub counter: i32,
  pub on_increment: Option<Callback<()>>,
  pub on_decrement: Option<Callback<()>>,
}

impl Component for Counter {
  fn render(props: Self) -> VNode {
    html(
      "div",
      [props::classnames("counter")],
      [
        html("h2", None, ["Counter: ".into(), props.counter.into()]),
        html(
          "button",
          [(
            "onClick",
            props
              .on_decrement
              .clone()
              .map(|on_decrement| -> JsValue {
                Callback::new(move |_: JsValue| on_decrement(())).into()
              })
              .into(),
          )],
          ["Decrement".into()],
        ),
        " ".into(),
        html(
          "button",
          [(
            "onClick",
            props
              .on_increment
              .clone()
              .map(|on_increment| -> JsValue {
                Callback::new(move |_: JsValue| on_increment(())).into()
              })
              .into(),
          )],
          ["Increment".into()],
        ),
      ],
    )
  }
}

impl HasJsComponent for Counter {
  type JsComponent = JsCounter;

  fn js_name() -> &'static str {
    "Counter"
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = Counter)]
pub struct JsCounter {
  component: Counter,
}

impl From<JsCounter> for Counter {
  fn from(value: JsCounter) -> Self {
    value.component
  }
}

impl From<Counter> for JsCounter {
  fn from(component: Counter) -> Self {
    Self { component }
  }
}

#[wasm_bindgen(js_class = Counter)]
impl JsCounter {
  #[wasm_bindgen]
  pub fn render(props: JsCounter) -> VNode {
    Counter::render(props.component)
  }
}
