//! This module provides convenience methods for building React props for
//! Javascript consumption.

mod attr;
mod classnames;
mod event;
mod props;
mod style;

pub use attr::*;
pub use classnames::*;
pub use event::*;
pub use props::*;
pub use style::*;

use crate::{create_element, VNode};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

/// The builder that powers [`h()`](crate::h).
pub struct H<'a> {
  pub(super) tag: &'a str,
  pub(super) props: Props,
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
  pub fn children(self, children: impl IntoIterator<Item = VNode>) -> VNode {
    create_element(&self.tag.into(), self.props, children)
  }

  /// Builds the [`VNode`] and returns it without any children.
  pub fn build(self) -> VNode {
    self.children([])
  }
}
