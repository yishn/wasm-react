use super::Props;
use crate::{children, create_element, VNode};
use js_sys::Array;
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

/// The builder that powers [`h()`](crate::h).
pub struct H<'a> {
  pub(crate) tag: &'a str,
  pub(crate) props: Props,
}

impl<'a> H<'a> {
  /// Sets an attribute on the [`VNode`].
  pub fn attr(mut self, key: &str, value: impl Into<JsValue>) -> Self {
    self.props = self.props.insert(key, value);
    self
  }

  /// Sets a callback value to an attribute on the [`VNode`].
  pub fn attr_callback<T, U>(
    mut self,
    key: &str,
    f: impl Fn(T) -> U + 'static,
  ) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    self.props = self.props.insert_callback(key, f);
    self
  }

  /// Builds the [`VNode`] and returns it with the given children.
  pub fn build_with(self, children: Array) -> VNode {
    create_element(&self.tag.into(), self.props, children)
  }

  /// Builds the [`VNode`] and returns it without any children.
  pub fn build(self) -> VNode {
    self.build_with(children![])
  }
}
