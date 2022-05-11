use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};

use crate::{callback::Callable, react_bindings};

#[derive(Clone)]
pub struct Transition {
  is_pending: bool,
  start_transition: Function,
}

impl Transition {
  pub fn is_pending(&self) -> bool {
    self.is_pending
  }

  pub fn start_transition(&self, f: impl FnOnce() + 'static) {
    self
      .start_transition
      .call(&Closure::once_into_js(f))
      .unwrap_throw();
  }
}

pub fn use_transition() -> Transition {
  let result = react_bindings::use_transition();

  let is_pending = result.get(0).as_bool().unwrap_throw();
  let start_transition = result.get(1).dyn_into::<Function>().unwrap_throw();

  Transition {
    is_pending,
    start_transition,
  }
}
