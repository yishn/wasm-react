use super::{use_ref, Deps, RefContainer, RefContainerRef};
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::UnwrapThrowExt;

pub struct MemoRef<T>(RefContainerRef<Option<T>>);

impl<T: 'static> Deref for MemoRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.deref().as_ref().unwrap_throw()
  }
}

pub struct Memo<T>(RefContainer<Option<T>>);

impl<T: 'static> Memo<T> {
  pub fn get(self) -> MemoRef<T> {
    MemoRef(self.0.current())
  }
}

impl<T> Debug for Memo<T>
where
  T: Debug + 'static,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Memo").field(&*self.get()).finish()
  }
}

impl<T> Clone for Memo<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Copy for Memo<T> {}

pub fn use_memo<T, D>(f: impl FnOnce() -> T, deps: Deps<D>) -> Memo<T>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let value_container = use_ref(|| None::<T>);
  let deps_container = use_ref(|| None::<Deps<D>>);

  let current_deps = deps_container.current();
  let need_update = deps.is_all() || Some(&deps) != current_deps.as_ref();

  if need_update {
    value_container.set_current(Some(f()));
    deps_container.set_current(Some(deps));
  }

  Memo(value_container)
}
