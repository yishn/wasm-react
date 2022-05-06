use crate::{react_bindings, Callable, Callback};
use js_sys::Function;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

pub struct UseState<T>(*mut T, Function);

impl<T: 'static> UseState<T> {
  pub fn update<'a>(&'a self, mutator: impl Fn(&'a mut T) + 'static) {
    let ptr = self.0;

    self.1.call(
      &Callback::new(move |_: JsValue| {
        let state = Box::leak(unsafe { Box::from_raw(ptr) });
        mutator(state);
      })
      .into(),
    );
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

impl<T: PartialEq> PartialEq for UseState<T> {
  fn eq(&self, other: &Self) -> bool {
    self.deref() == other.deref()
  }
}

impl<T: Eq> Eq for UseState<T> {}

impl<T: PartialOrd> PartialOrd for UseState<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.deref().partial_cmp(other.deref())
  }
}

impl<T: Ord> Ord for UseState<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.deref().cmp(other.deref())
  }
}

pub fn use_state<T: 'static>(value: impl Fn() -> T) -> UseState<T> {
  // The lifetime of the state (`T`) is completely managed by the React
  // component, meaning whenever the component is unmounted by React, the state
  // will also be dropped.

  let result = react_bindings::use_rust_state(
    &|| Box::into_raw(Box::new(value())) as usize,
    // This callback will be called when the component unmounts
    Callback::new(|ptr: usize| unsafe {
      drop(Box::from_raw(ptr as *mut T));
    })
    .into(),
  );
  let update_state = result.get(1).dyn_into::<Function>().unwrap_throw();
  let ptr = react_bindings::cast_into_usize(result.get(0)) as *mut T;

  UseState(ptr, update_state)
}
