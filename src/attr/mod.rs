mod classnames;
mod event;
mod style;

pub use classnames::*;
pub use event::*;
pub use style::*;

use crate::Callback;
use js_sys::{Object, Reflect};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

#[derive(Debug, Default, Clone)]
pub struct Attr(Object);

impl Attr {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(self, key: &str, value: impl Into<JsValue>) -> Self {
    Reflect::set(&self.0, &key.into(), &value.into()).unwrap();
    self
  }

  pub fn insert_callback<T, U>(
    self,
    key: &str,
    f: impl Fn(T) -> U + 'static,
  ) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    self.insert(key, Callback::new(f))
  }

  pub fn dangerously_set_inner_html(self, value: &str) -> Self {
    self.insert(
      "dangerouslySetInnerHTML",
      Attr::new().insert("__html", value),
    )
  }
}

impl From<Attr> for JsValue {
  fn from(style: Attr) -> Self {
    style.0.into()
  }
}
