use super::{use_memo, Deps, UseMemo};
use crate::callback::Callback;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};

pub fn use_callback<T, U, D>(
  f: impl FnMut(T) -> U + 'static,
  deps: Deps<D>,
) -> UseMemo<Callback<T, U>, D>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
  D: Eq + 'static,
{
  let callback = Callback::new(f);

  use_memo(move || callback.clone(), deps)
}
