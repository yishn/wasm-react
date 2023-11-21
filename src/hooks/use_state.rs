use super::{use_ref, RefContainer};
use crate::react_bindings;
use js_sys::Function;
use std::cell::Ref;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

/// Allows access to the underlying state data persisted with [`use_state()`].
#[derive(Debug)]
pub struct State<T> {
  ref_container: RefContainer<Option<T>>,
  update: Function,
}

impl<T: 'static> State<T> {
  /// Returns a reference to the value of the state.
  pub fn value(&self) -> Ref<'_, T> {
    Ref::map(self.ref_container.current(), |x| {
      x.as_ref().expect_throw("no state value available")
    })
  }

  /// Sets the state to the return value of the given mutator closure and
  /// rerenders the component.
  ///
  /// # Panics
  ///
  /// Panics if the value is currently borrowed.
  pub fn set(&mut self, mutator: impl FnOnce(T) -> T) {
    let value = self.ref_container.current_mut().take();
    let new_value = value.map(|value| mutator(value));

    self.ref_container.set_current(new_value);
    self
      .update
      .call0(&JsValue::NULL)
      .expect_throw("unable to call state update");
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
/// # struct State { greet: &'static str }
/// # struct C;
/// # impl C {
/// fn render(&self) -> VNode {
///   let state = use_state(|| State { greet: "Hello!" });
///
///   use_effect({
///     let mut state = state.clone();
///
///     move || {
///       state.set(|mut state| {
///         state.greet = "Welcome!";
///         state
///       });
///
///       || ()
///     }
///   }, Deps::some(( /* … */ )));
///
///   let vnode = h!(div).build(state.value().greet);
///   vnode
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
