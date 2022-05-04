use crate::Callback;
use js_sys::{Object, Reflect};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

/// A convenience builder for Javascript objects. Mainly used for constructing
/// props that are not controlled by Rust.
///
/// Use [`Attr`](super::Attr) to build props for HTML elements and
/// [`Style`](super::Style) to create style objects.
///
/// # Example
///
/// ```
/// Props::new()
///   .insert("id", "app")
///   .insert_callback("onClick", |_| console::log("Hello"))
/// ```
#[derive(Debug, Default, Clone)]
pub struct Props(Object);

impl Props {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self::default()
  }

  /// Equivalent to `props[key] = value;`.
  pub fn insert(self, key: &str, value: impl Into<JsValue>) -> Self {
    Reflect::set(&self.0, &key.into(), &value.into()).unwrap();
    self
  }

  /// Equivalent to `props[key] = f;`.
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
}

impl From<Props> for JsValue {
  fn from(style: Props) -> Self {
    style.0.into()
  }
}
