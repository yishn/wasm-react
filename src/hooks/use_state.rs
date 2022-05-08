use super::{use_ref, RefContainer};
use crate::{
  callback::Void, react_bindings, Callable, Persisted, PersistedOrigin,
};
use js_sys::Function;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::UnwrapThrowExt;

pub struct State<T>(RefContainer<Option<T>>, Function);

impl<T: 'static> State<T> {
  pub fn update(&mut self, mutator: impl FnOnce(&mut T)) {
    mutator(self.0.current_mut().as_mut().unwrap_throw());

    self.1.call(&Void.into()).unwrap_throw();
  }
}

impl<T> Persisted for State<T> {
  fn ptr(&self) -> PersistedOrigin {
    self.0.ptr()
  }
}

impl<T> Clone for State<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1.clone())
  }
}

impl<T: Debug> Debug for State<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

impl<T> Deref for State<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.current().as_ref().unwrap_throw()
  }
}

pub fn use_state<T: 'static>(init: impl FnOnce() -> T) -> State<T> {
  let mut ref_container = use_ref(None);

  if ref_container.current().is_none() {
    ref_container.set_current(Some(init()));
  }

  let update = react_bindings::use_rust_state();

  State(ref_container.into(), update)
}
