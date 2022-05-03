use super::Attr;
use wasm_bindgen::JsValue;

#[derive(Debug, Default, Clone)]
pub struct Style(Attr);

impl Style {
  pub fn new() -> Self {
    Self(Attr::new())
  }

  pub fn insert(self, key: &str, value: impl Into<JsValue>) -> Self {
    Self(self.0.insert(key, value.into()))
  }
}

impl From<Style> for JsValue {
  fn from(style: Style) -> Self {
    style.0.into()
  }
}

impl Attr {
  pub fn style(self, value: Style) -> Self {
    self.insert("style", value)
  }
}
