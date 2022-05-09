use crate::{callback::PersistedCallback, hooks::JsRefContainer};
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
/// # use wasm_react::{callback::*, props::*};
/// # use wasm_bindgen::prelude::*;
/// #
/// # fn f(handle_click: PersistedCallback<Void>) -> Props {
/// Props::new()
///   .insert("id", &"app".into())
///   .insert_callback("onClick", &handle_click)
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Props(Object);

impl Props {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the [React key][key].
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  pub fn key(self, value: Option<&str>) -> Self {
    self.insert("key", &value.into())
  }

  /// Sets the [React ref][ref] to the given ref container created with the
  /// [`use_js_ref()`](crate::hooks::use_js_ref()) hook.
  ///
  /// [ref]: https://reactjs.org/docs/refs-and-the-dom.html
  pub fn ref_container<T>(self, ref_container: &JsRefContainer<T>) -> Self {
    self.insert("ref", ref_container.as_ref())
  }

  /// Sets the [React ref][ref] to the given ref callback.
  ///
  /// [ref]: https://reactjs.org/docs/refs-and-the-dom.html
  pub fn ref_callback<T>(
    self,
    ref_callback: &PersistedCallback<T, ()>,
  ) -> Self {
    self.insert_callback("ref", ref_callback)
  }

  /// Equivalent to `props[key] = value;`.
  pub fn insert(self, prop: &str, value: &JsValue) -> Self {
    Reflect::set(&self.0, &prop.into(), value).unwrap_throw();
    self
  }

  /// Equivalent to `props[key] = f;`.
  pub fn insert_callback<T, U>(
    self,
    prop: &str,
    f: &PersistedCallback<T, U>,
  ) -> Self {
    Reflect::set(&self.0, &prop.into(), f.as_ref()).unwrap_throw();
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
