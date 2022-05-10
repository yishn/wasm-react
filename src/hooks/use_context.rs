use super::RefContainer;
use crate::{react_bindings, Context};
use std::{rc::Rc, thread::LocalKey};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub fn use_context<T>(context: &'static LocalKey<Context<T>>) -> Rc<T> {
  let js_ref =
    use_js_context(&context.with(|context| context.js_context.clone()));
  let ref_container = RefContainer::<Rc<T>>::try_from(js_ref).unwrap_throw();

  ref_container.current().clone()
}

pub fn use_js_context(context: &JsValue) -> JsValue {
  react_bindings::use_context(context)
}
