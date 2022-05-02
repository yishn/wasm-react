mod react;

use js_sys::{Function, Object, Reflect};
use react::get_component;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::{prelude::*, JsCast};
use HtmlOrComponent::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(x: &str);
}

pub enum HtmlOrComponent {
  HtmlTag(&'static str),
  Component(&'static str),
}

pub fn h(
  tag: HtmlOrComponent,
  props: impl IntoIterator<Item = (&'static str, JsValue)>,
  children: impl IntoIterator<Item = JsValue>,
) -> JsValue {
  let props_obj = Object::new();

  for (prop, value) in props.into_iter() {
    Reflect::set(&props_obj, &(*prop).into(), &value).ok();
  }

  react::create_element(
    match tag {
      HtmlTag(tag) => tag.into(),
      Component(name) => get_component(name),
    },
    props_obj.into(),
    children.into_iter().collect(),
  )
}

pub fn hc<C: Into<JsValue>>(name: &'static str, props: C) -> JsValue {
  h(Component(name), [("rustProps", props.into())], [])
}

#[derive(Debug)]
pub struct UseState<T>(*mut T, Function);

impl<T> Clone for UseState<T> {
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

impl<T> Deref for UseState<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }
}

impl<T: 'static> UseState<T> {
  pub fn update(&self, mutator: impl Fn(&mut T) + 'static) {
    let ptr = self.0;

    self
      .1
      .call1(
        &JsValue::undefined(),
        &Closure::wrap(Box::new(move || {
          let state = Box::leak(unsafe { Box::from_raw(ptr) });
          mutator(state);
        }) as Box<dyn Fn()>)
        .into_js_value(),
      )
      .ok();
  }
}

pub fn use_state<T: 'static>(value: impl Fn() -> T) -> UseState<T> {
  // The lifetime of the state (`T`) is completely managed by the React
  // component lifetime, meaning whenever the component gets removed from the
  // DOM by React, the state will also be dropped.

  let result = react::use_rust_state(
    &|| Box::into_raw(Box::new(value())) as usize as f64,
    Closure::wrap(Box::new(|ptr: f64| unsafe {
      drop(Box::from_raw(ptr as usize as *mut T));
    }) as Box<dyn Fn(f64)>)
    .into_js_value(),
  );
  let update_state = result.get(1).dyn_into::<Function>().unwrap();
  let ptr = result.get(0).as_f64().unwrap() as usize as *mut T;

  UseState(ptr, update_state)
}

#[derive(Debug)]
pub struct AppState {
  pub counter: i32,
}

impl Default for AppState {
  fn default() -> Self {
    Self { counter: 11 }
  }
}

#[wasm_bindgen]
pub struct App;

#[wasm_bindgen]
impl App {
  #[wasm_bindgen]
  pub fn render() -> JsValue {
    let state = use_state(|| AppState::default());

    h(
      HtmlTag("div"),
      None,
      [
        hc(
          "Counter",
          Counter {
            counter: state.counter,
          },
        ),
        h(
          HtmlTag("button"),
          [("onClick", {
            let state = state.clone();

            Closure::wrap(Box::new(move || {
              state.update(|state| {
                state.counter += 1;
              })
            }) as Box<dyn FnMut()>)
            .into_js_value()
          })],
          ["Increment".into()],
        ),
        " ".into(),
        h(
          HtmlTag("button"),
          [("onClick", {
            let state = state.clone();

            Closure::wrap(Box::new(move || {
              state.update(|state| {
                state.counter -= 1;
              })
            }) as Box<dyn FnMut()>)
            .into_js_value()
          })],
          ["Decrement".into()],
        ),
      ],
    )
  }
}

#[wasm_bindgen]
pub struct Counter {
  counter: i32,
}

#[wasm_bindgen]
impl Counter {
  #[wasm_bindgen]
  pub fn render(props: Counter) -> JsValue {
    h(
      HtmlTag("div"),
      [("className", "counter".into())],
      ["Counter: ".into(), props.counter.into()],
    )
  }
}

#[wasm_bindgen(start)]
pub fn main() {
  react::register_component("App");
  react::register_component("Counter");
}
