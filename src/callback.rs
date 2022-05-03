use std::{ops::Deref, rc::Rc};
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsValue};

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
