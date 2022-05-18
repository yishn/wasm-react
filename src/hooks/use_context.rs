use super::RefContainer;
use crate::{react_bindings, Context};
use std::{rc::Rc, thread::LocalKey};
use wasm_bindgen::UnwrapThrowExt;

/// Allows access to the current context value of the given context.
///
/// See [`create_context()`](crate::create_context()) for usage.
pub fn use_context<T>(context: &'static LocalKey<Context<T>>) -> Rc<T> {
  context.with(|context| {
    let js_ref = react_bindings::use_context(context.as_ref());
    let ref_container = RefContainer::<Rc<T>>::try_from(js_ref).unwrap_throw();
    let value = ref_container.current();

    value.clone()
  })
}
