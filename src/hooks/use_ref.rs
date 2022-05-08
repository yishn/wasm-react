use crate::{
  callback::{Callback, Void},
  react_bindings, Persisted, PersistedOrigin,
};
use js_sys::Reflect;
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

pub struct RefContainer<T>(*mut T);

impl<T> RefContainer<T> {
  pub fn current(&self) -> &T {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }

  pub fn current_mut(&mut self) -> &mut T {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }

  pub fn set_current(&mut self, value: T) {
    *self.current_mut() = value;
  }
}

impl<T> Persisted for RefContainer<T> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

impl<T: Debug> Debug for RefContainer<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RefContainer")
      .field("current", self.current())
      .finish()
  }
}

impl<T> Clone for RefContainer<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

pub fn use_ref<T: 'static>(init: T) -> RefContainer<T> {
  // The lifetime of the ref (`UseRefInner<T>`) is completely managed by the
  // React component. Whenever the component is unmounted by React, the state
  // will also be dropped.
  let ptr = react_bindings::use_rust_ref(
    Callback::once(move |_: Void| Box::into_raw(Box::new(init))).as_ref(),
    // This callback will always be called exactly one time. Either with
    // `Some(ptr)` when the component unmounts, at which point we should also
    // drop the inner value, or with `None` where we should do nothing.
    &Closure::once_into_js(move |ptr: Option<usize>| {
      if let Some(ptr) = ptr {
        drop(unsafe { Box::from_raw(ptr as *mut T) });
      }
    }),
  );

  RefContainer(ptr as *mut T)
}

pub struct JsRefContainer<T>(JsValue, PhantomData<T>);

impl<T: JsCast> JsRefContainer<Option<T>> {
  pub fn current(&self) -> Option<T> {
    Reflect::get(&self.0, &"current".into())
      .unwrap_throw()
      .dyn_into::<T>()
      .map(|t| Some(t))
      .unwrap_or(None)
  }

  pub fn set_current(&self, value: Option<&T>) {
    Reflect::set(
      &self.0,
      &"current".into(),
      value.map(|t| t.as_ref()).unwrap_or(&JsValue::null()),
    )
    .unwrap_throw();
  }
}

impl<T> Persisted for JsRefContainer<T> {
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

pub fn use_js_ref<T: JsCast>(init: Option<T>) -> JsRefContainer<Option<T>> {
  let ref_container = react_bindings::use_ref(
    &init.map(|init| init.into()).unwrap_or(JsValue::null()),
  );

  JsRefContainer(ref_container, PhantomData)
}
