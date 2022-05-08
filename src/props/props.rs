use crate::Callback;
use js_sys::{Object, Reflect};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

/// A convenience builder for JS objects. Mainly used for constructing props
/// that are not controlled by Rust.
///
/// Use [`Style`](super::Style) to create style objects which also provides
/// auto-completion.
///
/// # Example
///
/// ```
/// Props::new()
///   .insert("id", "app")
///   .insert_callback("onClick", handle_click)
/// ```
#[derive(Debug, Default, Clone)]
pub struct Props(Object);

impl Props {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self::default()
  }

  /// Equivalent to `props[key] = value;`.
  pub fn insert(self, key: &str, value: &JsValue) -> Self {
    Reflect::set(&self.0, &key.into(), value).unwrap_throw();
    self
  }

  /// Equivalent to `props[key] = f;`.
  pub fn insert_callback<T, U>(self, key: &str, f: &Callback<T, U>) -> Self {
    Reflect::set(&self.0, &key.into(), f.as_ref()).unwrap_throw();
    self
  }
}

impl AsRef<JsValue> for Props {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}

impl From<Props> for JsValue {
  fn from(style: Props) -> Self {
    style.0.into()
  }
}
