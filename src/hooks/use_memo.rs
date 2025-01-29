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
  pub fn get(&self) -> MemoRef<T> {
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

pub fn use_memo<T, D>(mut f: impl FnMut() -> T, deps: Deps<D>) -> Memo<T>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let deps_ref_container = use_ref(|| None::<Deps<D>>);
  let value_ref_container = use_ref(|| None::<T>);

  let need_update = {
    let current = deps_ref_container.current();
    let old_deps = current.as_ref();

    deps.is_all() || Some(&deps) != old_deps
  };

  if need_update {
    deps_ref_container.set_current(Some(deps));
    value_ref_container.set_current(Some(f()));
  }

  Memo(value_ref_container)
}
