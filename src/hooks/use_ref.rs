use crate::{
  callback::{Callback, Void},
  react_bindings,
};
use js_sys::Reflect;
use std::{
  fmt::{Debug, Pointer},
  marker::PhantomData,
  ops::{Deref, DerefMut},
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RefContainer<T>(T);

impl<T> RefContainer<T> {
  pub fn current(&self) -> &T {
    &self.0
  }

  pub fn current_mut(&mut self) -> &mut T {
    &mut self.0
  }

  pub fn set_current(&mut self, value: T) {
    self.0 = value;
  }
}

pub struct UseRef<T>(*mut RefContainer<T>);

impl<T> Debug for UseRef<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

impl<T> Clone for UseRef<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Deref for UseRef<T> {
  type Target = RefContainer<T>;

  fn deref(&self) -> &Self::Target {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }
}

impl<T> DerefMut for UseRef<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }
}

pub fn use_ref<T: 'static>(init: T) -> UseRef<T> {
  // The lifetime of the ref (`UseRefInner<T>`) is completely managed by the
  // React component. Whenever the component is unmounted by React, the state
  // will also be dropped.
  let ptr = react_bindings::use_rust_ref(
    Callback::once(move |_: Void| Box::into_raw(Box::new(init))).as_ref(),
    // This callback will always be called exactly one time. Either with
    // `Some(ptr)` when the component unmounts, at which point we should also
    // drop the inner value, or with `None` where we should do nothing.
    &Closure::once_into_js(move |ptr: Option<usize>| unsafe {
      if let Some(ptr) = ptr {
        drop(Box::from_raw(ptr as *mut RefContainer<T>));
      }
    }),
  );

  UseRef(ptr as *mut RefContainer<T>)
}

pub struct JsRefContainer<T>(JsValue, PhantomData<T>);

impl<T: JsCast> JsRefContainer<T> {
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

pub fn use_js_ref<T: JsCast>(init: Option<T>) -> JsRefContainer<T> {
  let ref_container = react_bindings::use_ref(
    &init.map(|init| init.into()).unwrap_or(JsValue::null()),
  );

  JsRefContainer(ref_container, PhantomData)
}
