use super::Props;
use crate::Callback;
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

/// To be used with [`Attr::dangerously_set_inner_html()`].
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DangerousHtml {
  pub __html: String,
}

/// A convenience wrapper around [`Props`] that provides auto-completion for
/// HTML attributes.
#[derive(Debug, Default, Clone)]
pub struct Attr(Props);

impl Attr {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self(Props::new())
  }

  /// Equivalent to `props[key] = value;`.
  pub fn insert(self, key: &str, value: impl Into<JsValue>) -> Self {
    Self(self.0.insert(key, value.into()))
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
    Self(self.0.insert(key, Callback::new(f)))
  }

  /// Equivalent to `props.dangerouslySetInnerHTML = { __html: value.__html };`.
  ///
  /// See also [React documentation](https://reactjs.org/docs/dom-elements.html#dangerouslysetinnerhtml).
  ///
  /// # Example
  ///
  /// ```
  /// fn create_markup() -> DangerousHtml<'static> {
  ///   DangerousHtml {
  ///     __html: "First &middot; Second".into()
  ///   }
  /// }
  ///
  /// html("div", Attr::new().dangerously_set_inner_html(create_markup()), [])
  /// ```
  pub fn dangerously_set_inner_html(self, value: DangerousHtml) -> Self {
    Self(self.0.insert(
      "dangerouslySetInnerHTML",
      Props::new().insert("__html", value.__html),
    ))
  }
}

impl From<Attr> for JsValue {
  fn from(attr: Attr) -> Self {
    attr.0.into()
  }
}
