use super::{use_ref, Deps, UseRef};
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::UnwrapThrowExt;

pub struct UseMemo<T, D: PartialEq>(UseRef<Option<(T, Deps<D>)>>);

impl<T, D: PartialEq> Deref for UseMemo<T, D> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.deref().current.as_ref().unwrap_throw().0
  }
}

impl<T: Debug, D: PartialEq> Debug for UseMemo<T, D> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.deref().current.as_ref().unwrap_throw().0.fmt(f)
  }
}

pub fn use_memo<T, D>(
  f: impl Fn() -> T + 'static,
  deps: Deps<D>,
) -> UseMemo<T, D>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let mut memo = use_ref(None::<(T, Deps<D>)>);

  let old_deps = memo.current.as_ref().map(|memo| &memo.1);
  if deps == Deps::All || Some(&deps) != old_deps {
    memo.current = Some((f(), deps));
  }

  UseMemo(memo)
}
