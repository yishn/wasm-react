use super::Deps;
use crate::{react_bindings, Callback};
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};

pub fn use_callback<T, U>(
  f: impl Fn(T) -> U + 'static,
  deps: Deps,
) -> Callback<T, U>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
{
  Callback::from_function(react_bindings::use_callback(
    &Callback::new(f).into(),
    &deps.into(),
  ))
}
