use super::{use_memo, Deps};
use crate::Callback;

/// Returns a memoized callback.
pub fn use_callback<T, U, D>(f: Callback<T, U>, deps: Deps<D>) -> Callback<T, U>
where
  T: 'static,
  U: 'static,
  D: PartialEq + 'static,
{
  let memo = use_memo(move || f, deps);
  let result = memo.value().clone();

  result
}
