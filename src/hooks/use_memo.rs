use super::{use_effect, use_state, Deps, UseState};
use std::{fmt::Debug, ops::Deref, rc::Rc};

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

pub fn use_memo<T>(
  f: impl Fn() -> T + 'static,
  deps: Deps,
) -> UseMemo<T>
where
  T: 'static,
{
  let rc_f = Rc::new(f);
  let memo = use_state(|| rc_f());

  use_effect(
    {
      let memo = memo.clone();

      move || {
        let f = rc_f.clone();
        memo.update(move |memo| *memo = f());
        || ()
      }
    },
    deps,
  );

  UseMemo(memo)
}
