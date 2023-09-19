use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};

use crate::{callback::Callable, react_bindings};

/// Allows access to the transition state.
#[derive(Debug, Clone)]
pub struct Transition {
  is_pending: bool,
  start_transition: Function,
}

impl Transition {
  /// Returns whether the transition is active or not.
  pub fn is_pending(&self) -> bool {
    self.is_pending
  }

  /// Marks the updates in the given closure as transitions.
  pub fn start(&mut self, f: impl FnOnce() + 'static) {
    self
      .start_transition
      .call(&Closure::once_into_js(f))
      .expect_throw("unable to call start function");
  }
}

/// Returns a stateful value for the pending state of the transition, and a
/// function to start it.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*, callback::*};
/// #
/// # fn render() -> VNode {
/// let count = use_state(|| 0);
/// let transition = use_transition();
///
/// let handle_click = use_callback({
///   let mut transition = transition.clone();
///
///   move |_| {
///     let mut count = count.clone();
///
///     transition.start(move || {
///       count.set(|c| c + 1);
///     });
///   }
/// }, Deps::none());
///
/// h!(div).build((
///   transition.is_pending().then(||
///     h!(div).build("Loading…")
///   ),
///   h!(button).on_click(&handle_click).build(()),
/// ))
/// # }
/// ```
pub fn use_transition() -> Transition {
  let result = react_bindings::use_transition();

  let is_pending = result
    .get(0)
    .as_bool()
    .expect_throw("unable to read pending state from transition");
  let start_transition = result
    .get(1)
    .dyn_into::<Function>()
    .expect_throw("unable to read start function from transition");

  Transition {
    is_pending,
    start_transition,
  }
}
