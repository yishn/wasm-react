mod classnames;
mod event;
mod style;

pub use classnames::*;
pub use event::*;
pub use style::*;

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

#[derive(Debug, Default, Clone)]
pub struct Attr {
  data: Object,
}

impl Attr {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(self, key: &str, value: impl Into<JsValue>) -> Self {
    Reflect::set(&self.data, &key.into(), &value.into()).unwrap();
    self
  }
}

impl From<Attr> for JsValue {
  fn from(style: Attr) -> Self {
    style.data.into()
  }
}
