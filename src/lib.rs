mod react;

use js_sys::{Array, Function, Object, Reflect};
use react::get_component;
use wasm_bindgen::{prelude::*, JsCast};
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

  let children_arr = Array::new();

  for element in children.into_iter() {
    children_arr.push(&element);
  }

  react::create_element(
    match tag {
      HtmlTag(tag) => tag.into(),
      Component(name) => get_component(name),
    },
    props_obj.into(),
    children_arr,
  )
}

pub fn use_state(
  value: impl Fn() -> JsValue,
) -> (JsValue, impl Fn(JsValue) + Clone) {
  let result = react::use_state(&value);
  let set_state = result.get(1).dyn_into::<Function>().unwrap();

  (result.get(0), move |value: JsValue| {
    set_state.call1(&JsValue::undefined(), &value).ok();
  })
}

#[wasm_bindgen]
pub struct App;

#[wasm_bindgen]
impl App {
  #[wasm_bindgen]
  pub fn render() -> JsValue {
    let (counter, set_counter) = use_state(|| 0f64.into());
    let counter = counter.as_f64().unwrap();

    h(
      HtmlTag("div"),
      None,
      [
        h(Component("Counter"), [("counter", counter.into())], None),
        h(
          HtmlTag("button"),
          [("onClick", {
            let set_counter = set_counter.clone();

            Closure::wrap(Box::new(move || set_counter((counter + 1.0).into()))
              as Box<dyn FnMut()>)
            .into_js_value()
          })],
          ["Increment".into()],
        ),
        " ".into(),
        h(
          HtmlTag("button"),
          [("onClick", {
            let set_counter = set_counter.clone();

            Closure::wrap(Box::new(move || set_counter((counter - 1.0).into()))
              as Box<dyn FnMut()>)
            .into_js_value()
          })],
          ["Decrement".into()],
        ),
      ],
    )
  }
}

#[wasm_bindgen]
pub struct Counter;

#[wasm_bindgen]
impl Counter {
  #[wasm_bindgen]
  pub fn render(props: JsValue) -> JsValue {
    let counter = Reflect::get(&props, &"counter".into()).unwrap();

    h(
      HtmlTag("div"),
      [("className", "counter".into())],
      ["Counter: ".into(), counter.into()],
    )
  }
}

#[wasm_bindgen(start)]
pub fn main() {
  react::register_component("App");
  react::register_component("Counter");
}
