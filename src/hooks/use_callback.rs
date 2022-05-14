use super::{use_memo, Deps};
use crate::callback::{Callback, PersistedCallback};

/// Returns a persisted, memoized callback.
pub fn use_callback<T, U, D>(
  f: impl FnMut(T) -> U + 'static,
  deps: Deps<D>,
) -> PersistedCallback<T, U>
where
  T: 'static,
  U: 'static,
  D: PartialEq + 'static,
{
  let memo = use_memo(move || Callback::new(f), deps);
  let value = memo.value();

  PersistedCallback(value.clone())
}
