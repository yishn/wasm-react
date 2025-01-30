use super::{use_ref, Deps, RefContainer, RefContainerRef};
use std::{fmt::Debug, ops::Deref};

pub struct MemoRef<T>(RefContainerRef<T>);

impl<T: 'static> Deref for MemoRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

pub struct Memo<T>(RefContainer<T>);

impl<T: 'static> Memo<T> {
  pub fn get(self) -> MemoRef<T> {
    MemoRef(self.0.current())
  }
}

impl<T> Debug for Memo<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Memo").field(&self.0).finish()
  }
}

impl<T> Clone for Memo<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Copy for Memo<T> {}

pub fn use_memo<T, D>(f: impl Fn() -> T, deps: Deps<D>) -> Memo<T>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let deps_container = use_ref(|| None::<Deps<D>>);
  let value_container = use_ref(|| f());

  let current_deps = deps_container.current();
  let need_update = deps.is_all() || Some(&deps) != current_deps.as_ref();

  if need_update {
    if current_deps.is_some() {
      value_container.set_current(f());
    }

    deps_container.set_current(Some(deps));
  }

  Memo(value_container)
}
