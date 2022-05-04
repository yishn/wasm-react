use super::{Attr, Props};
use wasm_bindgen::JsValue;

/// A convenience wrapper around [`Props`] that provides auto-completion for
/// style-related properties.
#[derive(Debug, Default, Clone)]
pub struct Style(Props);

impl Style {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self(Props::new())
  }

  /// Equivalent to `props[key] = value;`.
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
  /// Sets the `style` prop.
  pub fn style(self, value: Style) -> Self {
    self.insert("style", value)
  }
}
