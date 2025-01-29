use super::{get_owner, use_ref, RefContainer, RefContainerRef};
use crate::react_bindings::use_update;
use generational_box::GenerationalBox;
use js_sys::Function;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub struct StateRef<T>(RefContainerRef<Option<T>>);

impl<T: 'static> Deref for StateRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.deref().as_ref().unwrap_throw()
  }
}

pub struct State<T>(RefContainer<Option<T>>);

impl<T: 'static> State<T> {
  pub fn get(&self) -> StateRef<T> {
    StateRef(self.0.current())
  }
}

impl<T> Debug for State<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("State").field(&self.0).finish()
  }
}

impl<T> Clone for State<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Copy for State<T> {}

pub struct StateMut<T>(RefContainer<Option<T>>, GenerationalBox<Function>);

impl<T: 'static> StateMut<T> {
  pub fn set(&self, value: T) {
    self.0.set_current(Some(value));
    self.1.read().call0(&JsValue::NULL).unwrap_throw();
  }

  pub fn update(&self, updater: impl FnOnce(T) -> T) {
    let value = self.0.current_mut().take().unwrap_throw();
    self.set(updater(value));
  }

  pub fn lazy_set(&self, value: T)
  where
    T: PartialEq,
  {
    if self.0.current().as_ref() != Some(&value) {
      self.set(value);
    }
  }
}

impl<T> Debug for StateMut<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("StateMut")
      .field(&self.0)
      .field(&self.1)
      .finish()
  }
}

impl<T> Clone for StateMut<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1.clone())
  }
}

impl<T> Copy for StateMut<T> {}

pub fn use_state<T: 'static>(init: impl Fn() -> T) -> (State<T>, StateMut<T>) {
  let owner = get_owner();
  let ref_container = use_ref(|| Some(init()));
  let update = owner.insert(use_update());

  (State(ref_container), StateMut(ref_container, update))
}
