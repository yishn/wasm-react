use crate::Callback;
use wasm_bindgen::{convert::FromWasmAbi, JsValue};
use web_sys::MouseEvent;

fn event_handler<T: FromWasmAbi + 'static>(
  name: &str,
  f: impl Fn(T) + 'static,
) -> (&str, JsValue) {
  (name, Callback::new(f).into())
}

pub fn on_click(f: impl Fn(MouseEvent) + 'static) -> (&'static str, JsValue) {
  event_handler("onClick", f)
}
