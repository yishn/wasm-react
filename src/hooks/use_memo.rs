use super::{use_effect, use_state, Deps, UseState};
use std::{fmt::Debug, ops::Deref};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UseMemo<T>(UseState<T>);

impl<T> Deref for UseMemo<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.deref()
  }
}

impl<T: Debug> Debug for UseMemo<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

pub fn use_memo<T, F>(f: F, deps: Deps) -> UseMemo<T>
where
  T: 'static,
  F: Fn() -> T + Clone + 'static,
{
  let memo = use_state(|| f());

  use_effect(
    {
      let memo = memo.clone();

      move || {
        let f = f.clone();
        memo.update(move |memo| *memo = f());
        || ()
      }
    },
    deps,
  );

  UseMemo(memo)
}
