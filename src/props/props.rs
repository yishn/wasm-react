use crate::{callback::PersistedCallback, hooks::JsRefContainer, KeyType};
use js_sys::{Object, Reflect};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi},
  JsCast, JsValue, UnwrapThrowExt,
};

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
  pub fn key(self, value: Option<impl KeyType>) -> Self {
    self.insert(
      "key",
      &value.map(|x| x.into()).unwrap_or(JsValue::UNDEFINED),
    )
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
    ref_callback: &PersistedCallback<Option<T>>,
  ) -> Self
  where
    T: OptionFromWasmAbi + 'static,
  {
    self.insert_callback("ref", ref_callback)
  }

  /// Equivalent to `props[key] = value;`.
  pub fn insert(self, key: &str, value: &JsValue) -> Self {
    Reflect::set(&self.0, &key.into(), value)
      .expect_throw("cannot write into props object");
    self
  }

  /// Equivalent to `props[key] = f;`.
  pub fn insert_callback<T, U>(
    self,
    key: &str,
    f: &PersistedCallback<T, U>,
  ) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    self.insert(key, &f.as_js())
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

impl From<Object> for Props {
  fn from(value: Object) -> Self {
    Props(value)
  }
}

impl TryFrom<JsValue> for Props {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(Props(value.dyn_into::<Object>()?))
  }
}
