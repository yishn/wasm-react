use super::{use_ref, Deps, RefContainer};
use crate::{Persisted, PersistedOrigin, ValueContainer, ValueContainerRef};
use std::cell::Ref;
use wasm_bindgen::UnwrapThrowExt;

/// Allows access to the underlying memoized data persisted with [`use_memo()`].
#[derive(Debug)]
pub struct Memo<T, D>(RefContainer<Option<(T, Deps<D>)>>);

impl<T: 'static, D: 'static> Memo<T, D> {
  /// Returns a reference to the underlying memoized data.
  pub fn value(&self) -> Ref<'_, T> {
    Ref::map(self.0.current(), |x| {
      &x.as_ref().expect_throw("no memo data available").0
    })
  }
}

impl<T: 'static, D: PartialEq + 'static> Persisted for Memo<T, D> {
  fn ptr(&self) -> PersistedOrigin {
    self.0.ptr()
  }
}

impl<T, D> Clone for Memo<T, D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: 'static, D: 'static> ValueContainer<T> for Memo<T, D> {
  fn value(&self) -> ValueContainerRef<'_, T> {
    ValueContainerRef::Ref(self.value())
  }
}

/// Returns a persisted, memoized value.
///
/// This will recompute the value with the given closure whenever the given
/// dependencies has changed from last render. This optimization helps to avoid
/// expensive calculations on every render.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// #
/// # fn compute_expensive_value(a: (), b: ()) -> &'static str { "" }
/// # struct C { a: (), b:() };
/// # impl C {
/// fn render(&self) -> VNode {
///   let a = self.a;
///   let b = self.b;
///   let memo = use_memo(|| compute_expensive_value(a, b), Deps::some((a, b)));
///
///   h!(div).build(c![*memo.value()])
/// }
/// # }
/// ```
pub fn use_memo<T, D>(create: impl FnOnce() -> T, deps: Deps<D>) -> Memo<T, D>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let mut ref_container = use_ref(None::<(T, Deps<D>)>);
  let need_update = {
    let current = ref_container.current();
    let old_deps = current.as_ref().map(|memo| &memo.1);

    deps.is_all() || Some(&deps) != old_deps
  };

  if need_update {
    ref_container.set_current(Some((create(), deps)));
  }

  Memo(ref_container)
}
