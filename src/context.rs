use crate::react_bindings;
use std::rc::Rc;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Context<T: 'static> {
  pub(crate) fallback_value: Rc<T>,
  pub(crate) js_context: JsValue,
}

pub fn create_context<T>(init: T) -> Context<T> {
  Context {
    fallback_value: Rc::new(init),
    js_context: react_bindings::create_context(&JsValue::undefined()),
  }
}
