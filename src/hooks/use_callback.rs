use super::{use_memo, Deps};
use crate::callback::{Callback, PersistedCallback};
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};

pub fn use_callback<T, U, D>(
  f: impl FnMut(T) -> U + 'static,
  deps: Deps<D>,
) -> PersistedCallback<T, U>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
  D: PartialEq + 'static,
{
  let memo = use_memo(move || Callback::new(f), deps);

  PersistedCallback((*memo).clone())
}
