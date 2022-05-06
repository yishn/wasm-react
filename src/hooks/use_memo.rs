use super::{use_ref, Deps, UseRef};
use std::{fmt::Debug, ops::Deref, rc::Rc};

pub struct UseMemo<T, D: Eq>(UseRef<(T, Deps<D>)>);

impl<T, D: Eq> Clone for UseMemo<T, D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T, D: Eq> Deref for UseMemo<T, D> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.deref().current.0
  }
}

impl<T: Debug, D: Eq> Debug for UseMemo<T, D> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.deref().current.0.fmt(f)
  }
}

pub fn use_memo<T, D>(
  f: impl Fn() -> T + 'static,
  deps: Deps<D>,
) -> UseMemo<T, D>
where
  T: 'static,
  D: Eq + 'static,
{
  let f = Rc::new(f);
  let mut memo = use_ref((f(), deps.clone()));

  let old_deps = &memo.current.1;
  if deps == Deps::All || &deps != old_deps {
    memo.current = (f(), deps);
  }

  UseMemo(memo)
}
