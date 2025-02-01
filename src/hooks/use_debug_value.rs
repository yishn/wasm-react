use crate::react_bindings;
use wasm_bindgen::JsValue;

pub fn use_debug_value(value: impl Into<JsValue>) {
  react_bindings::use_debug_value(&value.into());
}
