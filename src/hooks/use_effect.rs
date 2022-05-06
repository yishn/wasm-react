use super::Deps;
use crate::{callback::Void, react_bindings, Callback};
use wasm_bindgen::JsValue;

pub fn use_effect<F, G>(f: F, deps: Deps)
where
  F: Fn() -> G + 'static,
  G: Fn() + 'static,
{
  react_bindings::use_effect(
    &Callback::new(move |_: Void| -> JsValue {
      let g = f();
      Callback::new(move |_: Void| g()).into()
    })
    .into(),
    &deps.into(),
  )
}
