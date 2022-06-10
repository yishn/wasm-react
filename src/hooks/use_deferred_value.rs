use super::{use_ref, RefContainer};
use crate::{react_bindings, ValueContainer, ValueContainerRef, Persisted};
use std::cell::Ref;
use wasm_bindgen::UnwrapThrowExt;

/// Allows access to the underlying deferred value persisted with
/// [`use_deferred_value()`].
#[derive(Debug)]
pub struct DeferredValue<T>(RefContainer<Option<(T, u8)>>);

impl<T: 'static> DeferredValue<T> {
  /// Returns a reference to the underlying deferred value.
  pub fn value(&self) -> Ref<'_, T> {
    Ref::map(self.0.current(), |x| {
      &x.as_ref().expect_throw("no deferred value available").0
    })
  }
}

impl<T: 'static> Persisted for DeferredValue<T> {
  fn ptr(&self) -> crate::PersistedOrigin {
    self.0.ptr()
  }
}

impl<T> Clone for DeferredValue<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: 'static> ValueContainer<T> for DeferredValue<T> {
  fn value(&self) -> ValueContainerRef<'_, T> {
    ValueContainerRef::Ref(self.value())
  }
}

/// Returns the given value, or in case of urgent updates, returns the previous
/// value given.
pub fn use_deferred_value<T: 'static>(value: T) -> DeferredValue<T> {
  let mut ref_container = use_ref(None::<(T, u8)>);

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
    ref_container.set_current(Some((value, deferred_counter)));
  }

  DeferredValue(ref_container)
}
