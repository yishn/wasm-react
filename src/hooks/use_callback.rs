use super::{use_memo, Deps};
use crate::callback::Callback;

/// Returns a memoized callback.
pub fn use_callback<T, U, D>(
  f: impl FnMut(T) -> U + 'static,
  deps: Deps<D>,
) -> Callback<T, U>
where
  T: 'static,
  U: 'static,
  D: PartialEq + 'static,
{
  let memo = use_memo(move || Callback::new(f), deps);
  let result = memo.value().clone();

  result
}
