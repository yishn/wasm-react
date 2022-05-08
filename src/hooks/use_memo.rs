use super::{use_ref, Deps, RefContainer};
use crate::{Persisted, PersistedOrigin};
use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::UnwrapThrowExt;

pub struct Memo<T, D: PartialEq>(RefContainer<Option<(T, Deps<D>)>>);

impl<T, D: PartialEq> Persisted for Memo<T, D> {
  fn ptr(&self) -> PersistedOrigin {
    self.0.ptr()
  }
}

impl<T: Debug, D: PartialEq> Debug for Memo<T, D> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

impl<T, D: PartialEq> Clone for Memo<T, D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T, D: PartialEq> Deref for Memo<T, D> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0.current().as_ref().unwrap_throw().0
  }
}

pub fn use_memo<T, D>(f: impl FnOnce() -> T, deps: Deps<D>) -> Memo<T, D>
where
  T: 'static,
  D: PartialEq + 'static,
{
  let mut ref_container = use_ref(None::<(T, Deps<D>)>);
  let old_deps = ref_container.current().as_ref().map(|memo| &memo.1);

  if deps == Deps::All || Some(&deps) != old_deps {
    ref_container.set_current(Some((f(), deps)));
  }

  Memo(ref_container)
}
