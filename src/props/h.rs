use super::Props;
use crate::{create_element, hooks::JsRefContainer, Callback, KeyType, VNode};
use std::borrow::Cow;
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi},
  intern, JsValue,
};

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct HtmlTag<'a>(pub &'a str);

impl AsRef<str> for HtmlTag<'_> {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

/// A marker trait for the component type that [`H`] is supposed to build.
///
/// Can either be `HtmlTag` or any imported component.
pub trait HType {
  /// Returns a reference to the [`JsValue`] of this component type.
  fn as_js(&self) -> Cow<'_, JsValue>;
}

impl HType for HtmlTag<'_> {
  fn as_js(&self) -> Cow<'_, JsValue> {
    Cow::Owned(intern(self.0).into())
  }
}

/// The component builder that powers [`h!`](crate::h!), which provides
/// convenience methods for adding props.
///
/// In case `T` is `HtmlTag`, [`H<T>`] also provides auto-completion for HTML
/// attributes and events.
#[derive(Debug, Clone)]
pub struct H<T> {
  pub(crate) typ: T,
  pub(crate) props: Props,
}

impl<T: HType> H<T> {
  /// Creates a new instance of [`H`]. It is recommended to use the
  /// [`h!`](crate::h!) macro instead.
  pub fn new(typ: T) -> Self {
    Self {
      typ,
      props: Props::new(),
    }
  }

  /// Sets the [React key][key].
  ///
  /// [key]: https://react.dev/learn/rendering-lists#keeping-list-items-in-order-with-key
  pub fn key(mut self, value: Option<impl KeyType>) -> Self {
    self.props = self.props.key(value);
    self
  }

  /// Sets the [React ref][ref] to the given ref container created with the
  /// [`use_js_ref()`](crate::hooks::use_js_ref()) hook.
  ///
  /// [ref]: https://react.dev/learn/manipulating-the-dom-with-refs
  pub fn ref_container<E>(mut self, ref_container: &JsRefContainer<E>) -> Self {
    self.props = self.props.ref_container(ref_container);
    self
  }

  /// Sets the [React ref][ref] to the given ref callback.
  ///
  /// [ref]: https://react.dev/learn/manipulating-the-dom-with-refs
  pub fn ref_callback<E>(mut self, ref_callback: &Callback<Option<E>>) -> Self
  where
    E: OptionFromWasmAbi + 'static,
  {
    self.props = self.props.ref_callback(ref_callback);
    self
  }

  /// Sets an attribute on the [`VNode`].
  pub fn attr(mut self, key: &str, value: &JsValue) -> Self {
    self.props = self.props.insert(key, value);
    self
  }

  /// Sets a callback value to an attribute on the [`VNode`].
  pub fn attr_callback<U, V>(mut self, key: &str, f: &Callback<U, V>) -> Self
  where
    U: FromWasmAbi + 'static,
    V: IntoWasmAbi + 'static,
  {
    self.props = self.props.insert_callback(key, f);
    self
  }

  /// Builds the [`VNode`] and returns it with the given children.
  pub fn build(self, children: impl Into<VNode>) -> VNode {
    create_element(&self.typ.as_js(), &self.props, children.into())
  }
}
