use super::use_ref;
use crate::react_bindings;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;

/// Returns the given value, or in case of urgent updates, returns the previous
/// value given.
pub fn use_deferred_value<T: 'static>(value: T) -> Rc<T> {
  let mut ref_container = use_ref(None::<(Rc<T>, u8)>);

  let deferred_counter = react_bindings::use_deferred_value(
    ref_container
      .current()
      .as_ref()
      .map(|current| current.1.wrapping_add(1))
      .unwrap_or(0),
  );

  if Some(deferred_counter)
    != ref_container.current().as_ref().map(|current| current.1)
  {
    // Deferred value changed
    ref_container.set_current(Some((Rc::new(value), deferred_counter)));
  }

  let current = ref_container.current();
  current
    .as_ref()
    .map(|current| current.0.clone())
    .unwrap_throw()
}
