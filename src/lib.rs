mod react;

use js_sys::{Function, Object, Reflect};
use std::{
  fmt::{Debug, Display},
  ops::Deref,
  rc::Rc,
};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  describe::WasmDescribe,
  prelude::*,
  JsCast,
};

pub struct VNode(JsValue);

impl From<VNode> for JsValue {
  fn from(value: VNode) -> Self {
    value.0
  }
}

impl<T> From<T> for VNode
where
  T: Display + Into<JsValue>,
{
  fn from(value: T) -> Self {
    VNode(value.into())
  }
}

impl WasmDescribe for VNode {
  fn describe() {
    JsValue::describe()
  }
}

impl IntoWasmAbi for VNode {
  type Abi = <JsValue as IntoWasmAbi>::Abi;

  fn into_abi(self) -> Self::Abi {
    self.0.into_abi()
  }
}

impl FromWasmAbi for VNode {
  type Abi = <JsValue as FromWasmAbi>::Abi;

  unsafe fn from_abi(js: Self::Abi) -> Self {
    VNode(JsValue::from_abi(js))
  }
}

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

pub fn component_into_vnode<P: Into<JsValue>>(
  name: &'static str,
  props: P,
) -> VNode {
  create_element(
    react::get_component(name),
    [("rustProps", props.into())],
    None,
  )
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
  // component lifetime, meaning whenever the component is unmounted by React,
  // the state will also be dropped.

  let result = react::use_rust_state(
    &|| Box::into_raw(Box::new(value())) as usize,
    // This callback will be called when the component unmounts
    Callback::new(|ptr: usize| unsafe {
      drop(Box::from_raw(ptr as *mut T));
    })
    .into(),
  );
  let update_state = result.get(1).dyn_into::<Function>().unwrap();
  let ptr = react::cast_into_usize(result.get(0)) as *mut T;

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
  pub fn render() -> VNode {
    let state = use_state(|| AppState::default());

    html(
      "div",
      [("className", "app".into())],
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
    component_into_vnode("Counter", value)
  }
}

#[wasm_bindgen]
impl Counter {
  #[wasm_bindgen]
  pub fn render(props: Counter) -> VNode {
    html(
      "div",
      [("className", "counter".into())],
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
