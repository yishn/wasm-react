use super::{use_ref, RefContainer};
use crate::react_bindings;
use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsValue};

#[derive(Debug, Clone, Copy)]
pub struct Transition {
  is_pending: bool,
  start_transition: RefContainer<Option<Function>>,
}

impl Transition {
  pub fn is_pending(self) -> bool {
    self.is_pending
  }

  pub fn start(self, f: impl FnMut() + 'static) {
    self
      .start_transition
      .current()
      .as_ref()
      .map(|start_transition| {
        start_transition
          .call1(&JsValue::undefined(), &Closure::new(f).into_js_value())
      });
  }
}

pub fn use_transition() -> Transition {
  let mut transition = Transition {
    is_pending: false,
    start_transition: use_ref(|| None),
  };

  react_bindings::use_transition(&mut |is_pending, start_transition| {
    transition.is_pending = is_pending;
    transition
      .start_transition
      .set_current(Some(start_transition));
  });

  transition
}
