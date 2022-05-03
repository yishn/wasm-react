mod react;

use js_sys::{Function, Object, Reflect};
use react::get_component;
use std::{fmt::Debug, ops::Deref, rc::Rc};
use wasm_bindgen::{convert::FromWasmAbi, prelude::*, JsCast};
use HtmlOrComponent::*;

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

#[derive(Clone)]
pub struct Callback<T>(Rc<dyn Fn(T)>);

impl<T> Deref for Callback<T> {
  type Target = Rc<dyn Fn(T)>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: FromWasmAbi + 'static> From<Callback<T>> for JsValue {
  fn from(value: Callback<T>) -> Self {
    Closure::wrap(Box::new(move |arg| {
      value.0(arg);
    }) as Box<dyn Fn(T)>)
    .into_js_value()
  }
}

impl<T> Callback<T> {
  pub fn new<F: Fn(T) + 'static>(f: F) -> Self {
    Self(Rc::new(f))
  }
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
        &Callback::new(move |_: JsValue| {
          let state = Box::leak(unsafe { Box::from_raw(ptr) });
          mutator(state);
        })
        .into(),
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
    Callback::new(|ptr: f64| unsafe {
      drop(Box::from_raw(ptr as usize as *mut T));
    })
    .into(),
  );
  let update_state = result.get(1).dyn_into::<Function>().unwrap();
  let ptr = result.get(0).as_f64().unwrap() as usize as *mut T;

  UseState(ptr, update_state)
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
  pub fn render() -> JsValue {
    let state = use_state(|| AppState::default());

    hc(
      "Counter",
      Counter {
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
      },
    )
  }
}

#[wasm_bindgen]
pub struct Counter {
  counter: i32,
  on_increment: Callback<()>,
  on_decrement: Callback<()>,
}

#[wasm_bindgen]
impl Counter {
  #[wasm_bindgen]
  pub fn render(props: Counter) -> JsValue {
    h(
      HtmlTag("div"),
      None,
      [
        h(
          HtmlTag("div"),
          [("className", "counter".into())],
          ["Counter: ".into(), props.counter.into()],
        ),
        h(
          HtmlTag("button"),
          [("onClick", {
            let on_decrement = props.on_decrement.clone();

            Callback::new(move |_: JsValue| on_decrement(())).into()
          })],
          ["Decrement".into()],
        ),
        " ".into(),
        h(
          HtmlTag("button"),
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
