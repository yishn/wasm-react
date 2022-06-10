use super::{use_ref, Deps, RefContainer};
use crate::{Persisted, PersistedOrigin};
use std::cell::Ref;
use wasm_bindgen::UnwrapThrowExt;

/// Allows access to the underlying memoized data persisted with [`use_memo()`].
#[derive(Debug)]
pub struct Memo<T>(RefContainer<Option<T>>);

impl<T: 'static> Memo<T> {
  /// Returns a reference to the underlying memoized data.
  pub fn value(&self) -> Ref<'_, T> {
    Ref::map(self.0.current(), |x| {
      x.as_ref().expect_throw("no memo data available")
    })
  }
}

impl<T: 'static> Persisted for Memo<T> {
  fn ptr(&self) -> PersistedOrigin {
    self.0.ptr()
  }
}

impl<T> Clone for Memo<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
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
pub fn use_memo<T, D>(create: impl FnOnce() -> T, deps: Deps<D>) -> Memo<T>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let mut deps_ref_container = use_ref(None::<Deps<D>>);
  let mut value_ref_container = use_ref(None::<T>);

  let need_update = {
    let current = deps_ref_container.current();
    let old_deps = current.as_ref();

    deps.is_all() || Some(&deps) != old_deps
  };

  if need_update {
    deps_ref_container.set_current(Some(deps));
    value_ref_container.set_current(Some(create()));
  }

  Memo(value_ref_container)
}
