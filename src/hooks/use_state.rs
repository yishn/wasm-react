use super::{use_ref, RefContainer};
use crate::{
  callback::{Callable, Void},
  react_bindings, Persisted, PersistedOrigin,
};
use js_sys::Function;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::UnwrapThrowExt;

/// Allows access to the underlying state data persisted with [`use_state()`].
///
/// When the component unmounts, the underlying data is dropped. After that,
/// trying to access the data will result in a panic.
pub struct State<T> {
  ref_container: RefContainer<Option<T>>,
  update: Function,
}

impl<T: 'static> State<T> {
  /// Sets the state to the return value of the given mutator closure and
  /// rerenders the component.
  pub fn set(&mut self, mutator: impl FnOnce(&T) -> T) {
    let new_state =
      mutator(self.ref_container.current().as_ref().unwrap_throw());

    self.ref_container.set_current(Some(new_state));
    self.update.call(&Void.into()).unwrap_throw();
  }

  /// Updates the state with the given mutator closure and rerenders the
  /// component.
  pub fn update(&mut self, mutator: impl FnOnce(&mut T)) {
    mutator(self.ref_container.current_mut().as_mut().unwrap_throw());

    self.update.call(&Void.into()).unwrap_throw();
  }
}

impl<T> Persisted for State<T> {
  fn ptr(&self) -> PersistedOrigin {
    self.ref_container.ptr()
  }
}

impl<T> Clone for State<T> {
  fn clone(&self) -> Self {
    Self {
      ref_container: self.ref_container.clone(),
      update: self.update.clone(),
    }
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
    self.ref_container.current().as_ref().unwrap_throw()
  }
}

/// Persist stateful data of the component.
///
/// Unlike the [`use_ref()`] hook, updating the state will automatically trigger
/// a rerender of the component.
///
/// Unlike its React counterpart, setting the state will mutate the underlying
/// data immediately.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// #
/// # struct State { value: &'static str }
/// # struct C;
/// # impl C {
/// fn render(&self) -> VNode {
///   let state = use_state(|| State { value: "Hello!" });
///
///   use_effect({
///     let mut state = state.clone();
///
///     move || {
///       state.set(|_| State { value: "Welcome!" });
///       || ()
///     }
///   }, Deps::some(( /* ... */ )));
///
///   h!(div).build(c![state.value])
/// }
/// # }
/// ```
pub fn use_state<T: 'static>(init: impl FnOnce() -> T) -> State<T> {
  let mut ref_container = use_ref(None);

  if ref_container.current().is_none() {
    ref_container.set_current(Some(init()));
  }

  let update = react_bindings::use_rust_state();

  State {
    ref_container,
    update,
  }
}
