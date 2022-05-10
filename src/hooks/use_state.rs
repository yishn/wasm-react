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
pub struct State<T>(RefContainer<Option<T>>, Function);

impl<T: 'static> State<T> {
  /// Updates the underlying state with the given mutator closure and rerenders
  /// the component.
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

/// Persist stateful data of the component.
///
/// Unlike the [`use_ref()`] hook, updating the state will automatically trigger
/// a rerender of the component.
///
/// Unlike its React counterpart, calling `update` will mutate the underlying
/// data in-place and immediately.
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
///       state.update(|state| state.value = "Welcome!");
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

  State(ref_container.into(), update)
}
