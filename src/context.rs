use crate::react_bindings;
use std::rc::Rc;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Context<T> {
  pub(crate) fallback_value: Rc<T>,
  pub(crate) js_context: JsValue,
}

impl<T> AsRef<JsValue> for Context<T> {
  fn as_ref(&self) -> &JsValue {
    &self.js_context
  }
}

impl<T> From<Context<T>> for JsValue {
  fn from(value: Context<T>) -> Self {
    value.js_context
  }
}

pub fn create_context<T: 'static>(init: T) -> Context<T> {
  Context {
    fallback_value: Rc::new(init),
    js_context: react_bindings::create_context(&JsValue::undefined()),
  }
}
