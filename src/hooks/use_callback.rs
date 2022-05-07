use super::{use_memo, Deps};
use crate::callback::Callback;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};

pub fn use_callback<T, U, D>(
  f: impl FnMut(T) -> U + 'static,
  deps: Deps<D>,
) -> Callback<T, U>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
  D: PartialEq + 'static,
{
  let callback = Callback::new(f);
  let memo = use_memo(move || callback.clone(), deps);

  memo.clone()
}
