use crate::react_bindings;
use std::rc::Rc;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Context<T: 'static> {
  pub(crate) fallback_value: Rc<T>,
  pub(crate) js_context: JsValue,
}

impl<T: 'static> Clone for Context<T> {
  fn clone(&self) -> Self {
    Self {
      fallback_value: self.fallback_value.clone(),
      js_context: self.js_context.clone(),
    }
  }
}

pub fn create_context<T>(init: T) -> Context<T> {
  Context {
    fallback_value: Rc::new(init),
    js_context: react_bindings::create_context(&JsValue::undefined()),
  }
}
