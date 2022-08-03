use crate::{react_bindings, Persisted, PersistedOrigin};
use js_sys::Reflect;
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt, intern};

/// Allows access to the underlying JS data persisted with [`use_js_ref()`].
pub struct JsRefContainer<T>(JsValue, PhantomData<T>);

impl<T: JsCast> JsRefContainer<T> {
  /// Returns the underlying typed JS data.
  pub fn current(&self) -> Option<T> {
    self.current_untyped().dyn_into::<T>().ok()
  }

  /// Returns the underlying JS data as [`JsValue`].
  pub fn current_untyped(&self) -> JsValue {
    Reflect::get(&self.0, &intern("current").into())
      .expect_throw("cannot read from ref container")
  }

  /// Sets the underlying JS data.
  pub fn set_current(&self, value: Option<&T>) {
    Reflect::set(
      &self.0,
      &intern("current").into(),
      value.map(|t| t.as_ref()).unwrap_or(&JsValue::null()),
    )
    .expect_throw("cannot write into ref container");
  }
}

impl<T: 'static> Persisted for JsRefContainer<T> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

impl<T> Debug for JsRefContainer<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("JsRefContainer").field(&self.0).finish()
  }
}

impl<T> Clone for JsRefContainer<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), PhantomData)
  }
}

impl<T> AsRef<JsValue> for JsRefContainer<T> {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}

impl<T> From<JsRefContainer<T>> for JsValue {
  fn from(value: JsRefContainer<T>) -> Self {
    value.0
  }
}

impl<T> From<JsValue> for JsRefContainer<T> {
  fn from(value: JsValue) -> Self {
    Self(value, PhantomData)
  }
}

/// This hook can persist JS data through the entire lifetime of the component.
///
/// Use this if you need JS to set the ref value. If you only need to mutate the
/// data from Rust, use [`use_ref()`](crate::hooks::use_ref()) instead.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// # struct MyComponent;
/// impl Component for MyComponent {
///   fn render(&self) -> VNode {
///     let input_element = use_js_ref(None);
///
///     h!(div)
///       .build(c![
///         h!(input)
///           .ref_container(&input_element)
///           .html_type("text")
///           .build(c![])
///       ])
///   }
/// }
/// ```
pub fn use_js_ref<T: JsCast>(init: Option<T>) -> JsRefContainer<T> {
  let ref_container = react_bindings::use_ref(
    &init.map(|init| init.into()).unwrap_or(JsValue::null()),
  );

  JsRefContainer(ref_container, PhantomData)
}
