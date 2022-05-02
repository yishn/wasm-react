mod react;

use js_sys::{Function, Object, Reflect};
use react::get_component;
use std::fmt::Debug;
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

pub fn use_state<'a, T>(
  value: impl Fn() -> T,
) -> (&'a T, impl Fn(fn(&mut T)) + Clone)
where
  T: 'static,
{
  let result = react::use_rust_state(
    &|| Box::into_raw(Box::new(value())) as usize as f64,
    Closure::wrap(Box::new(|ptr: f64| unsafe {
      drop(Box::from_raw(ptr as usize as *mut T));
    }) as Box<dyn Fn(f64)>)
    .into_js_value(),
  );
  let update_state = result.get(1).dyn_into::<Function>().unwrap();
  let ptr = result.get(0).as_f64().unwrap() as usize as *mut T;
  let state = Box::leak(unsafe { Box::from_raw(ptr) });

  (state, move |mutator| {
    update_state
      .call1(
        &JsValue::undefined(),
        &Closure::wrap(Box::new(move |ptr: f64| {
          let state =
            Box::leak(unsafe { Box::from_raw(ptr as usize as *mut T) });
          mutator(state);
        }) as Box<dyn Fn(f64)>)
        .into_js_value(),
      )
      .ok();
  })
}

pub trait IntoJs {
  fn into_js(self);
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
    let (state, update_state) = use_state(|| AppState::default());

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
            let update_state = update_state.clone();

            Closure::wrap(Box::new(move || {
              update_state(|state| {
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
            let update_state = update_state.clone();

            Closure::wrap(Box::new(move || {
              update_state(|state| {
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
