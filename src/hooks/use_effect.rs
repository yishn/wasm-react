use crate::{react, Callback};
use js_sys::Array;
use wasm_bindgen::JsValue;

pub fn use_effect<G: Fn() + 'static, F: Fn() -> G + 'static>(
  f: F,
  deps: Option<&[JsValue]>,
) {
  react::use_effect(
    Callback::new(move |_: JsValue| -> JsValue {
      let g = f();
      Callback::new(move |_: JsValue| g()).into()
    })
    .into(),
    deps.map(|deps| deps.iter().collect::<Array>()),
  )
}
