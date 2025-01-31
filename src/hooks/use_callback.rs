use super::{use_memo, Deps, Memo};
use crate::Callback;

pub fn use_callback<T, U, D>(
  f: impl FnMut(T) -> U + 'static,
  deps: Deps<D>,
) -> Memo<Callback<T, U>>
where
  T: 'static,
  U: 'static,
  D: PartialEq + 'static,
{
  use_memo(move || Callback::new(f), deps)
}
