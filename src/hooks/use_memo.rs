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

pub fn use_memo<T: 'static, D: Eq + 'static>(
  f: impl Fn() -> T + 'static,
  deps: Deps<D>,
) -> UseMemo<T, D> {
  let rc_f = Rc::new(f);
  let memo = use_state({
    let f = rc_f.clone();
    let deps = deps.clone();
    move || (f(), deps)
  });

  if deps != memo.1 {
    memo.update({
      let f = rc_f.clone();
      move |memo| *memo = (f(), deps)
    });
  }

  UseMemo(memo)
}
