use crate::{
  callback::{Callback, Void},
  react_bindings, Callable,
};
use js_sys::Function;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};

pub struct UseState<T>(*mut T, Function);

impl<T: 'static> UseState<T> {
  pub fn update(&self, mutator: impl FnOnce(&mut T)) {
    let state = unsafe { Box::from_raw(self.0) };
    mutator(Box::leak(state));

    self.1.call(&Void.into());
  }
}

impl<T> Clone for UseState<T> {
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

impl<T> Deref for UseState<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }
}

impl<T: Debug> Debug for UseState<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

pub fn use_state<T: 'static>(
  default_value: impl FnOnce() -> T + 'static,
) -> UseState<T> {
  // The lifetime of the state (`T`) is completely managed by the React
  // component, meaning whenever the component is unmounted by React, the state
  // will also be dropped.

  let result = react_bindings::use_rust_state(
    Callback::once(move |_: Void| {
      Box::into_raw(Box::new(default_value())) as usize
    })
    .as_ref(),
    // This callback will be called when the component unmounts
    &Closure::once_into_js(|ptr: usize| unsafe {
      drop(Box::from_raw(ptr as *mut T));
    }),
  );

  let update_state = result.get(1).dyn_into::<Function>().unwrap_throw();
  let ptr = react_bindings::cast_into_usize(&result.get(0)) as *mut T;

  UseState(ptr, update_state)
}
