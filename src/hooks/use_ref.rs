use crate::{
  callback::{Callback, Void},
  react_bindings,
};
use std::{
  fmt::{Debug, Pointer},
  ops::{Deref, DerefMut},
};
use wasm_bindgen::prelude::Closure;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UseRefInner<T> {
  pub current: T,
}

pub struct UseRef<T>(*mut UseRefInner<T>);

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
  type Target = UseRefInner<T>;

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
    Callback::once(move |_: Void| Box::into_raw(Box::new(init)))
      .as_ref(),
    // This callback will always be called exactly one time. Either with
    // `Some(ptr)` when the component unmounts, at which point we should also
    // drop the inner value, or with `None` where we should do nothing.
    &Closure::once_into_js(move |ptr: Option<usize>| unsafe {
      if let Some(ptr) = ptr {
        drop(Box::from_raw(ptr as *mut UseRefInner<T>));
      }
    }),
  );

  UseRef(ptr as *mut UseRefInner<T>)
}
