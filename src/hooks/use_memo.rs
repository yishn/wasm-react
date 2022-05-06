use super::{use_state, Deps, UseState};
use std::{fmt::Debug, ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct UseMemo<T, D: Eq>(UseState<(T, Deps<D>)>);

impl<T, D: Eq> Deref for UseMemo<T, D> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.deref().0
  }
}

impl<T: Debug, D: Eq> Debug for UseMemo<T, D> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.deref().0.fmt(f)
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
  let memo = use_state({
    let f = f.clone();
    let deps = deps.clone();
    move || (f(), deps)
  });

  let old_deps = &memo.1;
  if deps == Deps::All || &deps != old_deps {
    memo.update(move |memo| *memo = (f(), deps));
  }

  UseMemo(memo)
}
