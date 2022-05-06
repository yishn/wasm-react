use super::{use_ref, Deps, UseRef};
use std::{fmt::Debug, ops::Deref, rc::Rc};

pub struct UseMemo<T, D: PartialEq>(UseRef<(T, Deps<D>)>);

impl<T, D: PartialEq> Deref for UseMemo<T, D> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.deref().current.0
  }
}

impl<T: Debug, D: PartialEq> Debug for UseMemo<T, D> {
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
  D: PartialEq + 'static,
{
  let f = Rc::new(f);
  let mut memo = use_ref((f(), deps.clone()));

  let old_deps = &memo.current.1;
  if deps == Deps::All || &deps != old_deps {
    memo.current = (f(), deps);
  }

  UseMemo(memo)
}
