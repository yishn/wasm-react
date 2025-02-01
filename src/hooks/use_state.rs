use super::{use_ref, RefContainer, RefContainerRef};
use crate::react_bindings::use_update;
use js_sys::Function;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub struct StateRef<T>(RefContainerRef<Option<(T, Function)>>);

impl<T: 'static> Deref for StateRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.deref().as_ref().unwrap_throw().0
  }
}

pub struct State<T>(RefContainer<Option<(T, Function)>>);

impl<T: 'static> State<T> {
  pub fn get(self) -> StateRef<T> {
    StateRef(self.0.current())
  }
}

impl<T> Debug for State<T>
where
  T: Debug + 'static,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("State").field(&*self.get()).finish()
  }
}

impl<T> Clone for State<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Copy for State<T> {}

pub struct StateMut<T>(RefContainer<Option<(T, Function)>>);

impl<T: 'static> StateMut<T> {
  pub fn set(self, value: T) {
    self.0.current_mut().as_mut().unwrap_throw().0 = value;
    self
      .0
      .current()
      .as_ref()
      .unwrap_throw()
      .1
      .call0(&JsValue::NULL)
      .unwrap_throw();
  }

  pub fn update(self, updater: impl FnOnce(T) -> T) {
    let (value, f) = self.0.current_mut().take().unwrap_throw();
    self.0.set_current(Some((updater(value), f)));
    self
      .0
      .current()
      .as_ref()
      .unwrap_throw()
      .1
      .call0(&JsValue::NULL)
      .unwrap_throw();
  }

  pub fn lazy_set(self, value: T)
  where
    T: PartialEq,
  {
    if self.0.current().as_ref().map(|(value, _)| value) != Some(&value) {
      self.set(value);
    }
  }
}

impl<T> Clone for StateMut<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Copy for StateMut<T> {}

pub fn use_state<T: 'static>(
  init: impl FnOnce() -> T,
) -> (State<T>, StateMut<T>) {
  let update = use_update();
  let ref_container = use_ref(move || Some((init(), update)));

  (State(ref_container), StateMut(ref_container))
}
