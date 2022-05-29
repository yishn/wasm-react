use super::Props;
use crate::{
  callback::PersistedCallback, create_element, hooks::JsRefContainer, VNode,
  VNodeList,
};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};
use web_sys::Element;

/// The builder that powers [`h!`](crate::h!). This provides auto-completion for
/// HTML attributes and events.
pub struct H<'a> {
  pub(crate) tag: &'a str,
  pub(crate) props: Props,
}

impl<'a> H<'a> {
  /// Creates a new instance of [`H`]. It is recommended to use the
  /// [`h!`](crate::h!) macro instead.
  pub fn new(tag: &'a str) -> Self {
    Self {
      tag,
      props: Props::new(),
    }
  }

  /// Sets the [React key][key].
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  pub fn key(mut self, value: Option<&str>) -> Self {
    self.props = self.props.key(value);
    self
  }

  /// Sets the [React ref][ref] to the given ref container created with the
  /// [`use_js_ref()`](crate::hooks::use_js_ref()) hook.
  ///
  /// [ref]: https://reactjs.org/docs/refs-and-the-dom.html
  pub fn ref_container(
    mut self,
    ref_container: &JsRefContainer<Element>,
  ) -> Self {
    self.props = self.props.ref_container(ref_container);
    self
  }

  /// Sets the [React ref][ref] to the given ref callback.
  ///
  /// [ref]: https://reactjs.org/docs/refs-and-the-dom.html
  pub fn ref_callback(
    mut self,
    ref_callback: &PersistedCallback<Option<Element>>,
  ) -> Self {
    self.props = self.props.ref_callback(ref_callback);
    self
  }

  /// Sets an attribute on the [`VNode`].
  pub fn attr(mut self, key: &str, value: &JsValue) -> Self {
    self.props = self.props.insert(key, value);
    self
  }

  /// Sets a callback value to an attribute on the [`VNode`].
  pub fn attr_callback<T, U>(
    mut self,
    key: &str,
    f: &PersistedCallback<T, U>,
  ) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    self.props = self.props.insert_callback(key, f);
    self
  }

  /// Builds the [`VNode`] and returns it with the given children. Use
  /// [`c!`](crate::c!) for easier construction of the children.
  pub fn build(self, children: VNodeList) -> VNode {
    create_element(&self.tag.into(), &self.props, children)
  }
}
