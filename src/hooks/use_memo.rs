use super::{use_ref, Deps, RefContainer};
use crate::{Persisted, PersistedOrigin};
use std::{
  fmt::Debug,
  ops::{Deref, DerefMut},
};
use wasm_bindgen::UnwrapThrowExt;

/// Allows access to the underlying memoized data persisted with [`use_memo()`].
pub struct Memo<T, D: PartialEq>(RefContainer<Option<(T, Deps<D>)>>);

impl<T, D: PartialEq> Persisted for Memo<T, D> {
  fn ptr(&self) -> PersistedOrigin {
    self.0.ptr()
  }
}

impl<T: Debug, D: PartialEq> Debug for Memo<T, D> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

impl<T, D: PartialEq> Clone for Memo<T, D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T, D: PartialEq> Deref for Memo<T, D> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.current().as_ref().unwrap_throw().0
  }
}

impl<T, D: PartialEq> DerefMut for Memo<T, D> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0.current_mut().as_mut().unwrap_throw().0
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
///   h!(div).build(children![*memo])
/// }
/// # }
/// ```
pub fn use_memo<T, D>(create: impl FnOnce() -> T, deps: Deps<D>) -> Memo<T, D>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let mut ref_container = use_ref(None::<(T, Deps<D>)>);
  let old_deps = ref_container.current().as_ref().map(|memo| &memo.1);

  if deps.is_all() || Some(&deps) != old_deps {
    ref_container.set_current(Some((create(), deps)));
  }

  Memo(ref_container)
}
