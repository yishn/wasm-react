use super::{use_ref, RefContainer, RefContainerRef};
use crate::react_bindings;
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::UnwrapThrowExt;

pub struct DeferredValueRef<T>(RefContainerRef<Option<(T, u8)>>);

impl<T: 'static> Deref for DeferredValueRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.as_ref().unwrap_throw().0
  }
}

pub struct DeferredValue<T>(RefContainer<Option<(T, u8)>>);

impl<T: 'static> DeferredValue<T> {
  pub fn get(self) -> DeferredValueRef<T> {
    DeferredValueRef(self.0.current())
  }
}

impl<T> Debug for DeferredValue<T>
where
  T: Debug + 'static,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("DeferredValue").field(&*self.get()).finish()
  }
}

impl<T> Clone for DeferredValue<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Copy for DeferredValue<T> {}

fn use_deferred_value_inner<T>(
  value: impl FnOnce(bool) -> T,
) -> DeferredValue<T>
where
  T: PartialEq + 'static,
{
  let ref_container = use_ref(|| None::<(T, u8)>);
  let value = value(ref_container.current().is_none());
  let new_count = ref_container
    .current()
    .as_ref()
    .map(|(old_value, count)| {
      if &value == old_value {
        *count
      } else {
        count.wrapping_add(1)
      }
    })
    .unwrap_or(1);

  let deferred_count = react_bindings::use_deferred_value(new_count, 0u8);

  if deferred_count == 0 || new_count == deferred_count {
    ref_container.set_current(Some((value, deferred_count)));
  }

  DeferredValue(ref_container)
}

pub fn use_deferred_value<T>(value: T) -> DeferredValue<T>
where
  T: PartialEq + 'static,
{
  use_deferred_value_inner(move |_| value)
}

pub fn use_deferred_value_with_init<T>(value: T, init: T) -> DeferredValue<T>
where
  T: PartialEq + 'static,
{
  use_deferred_value_inner(move |first| if first { init } else { value })
}
