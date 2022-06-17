use crate::{react_bindings, Context};
use std::{rc::Rc, thread::LocalKey, mem::ManuallyDrop};

/// Allows access to the current context value of the given context.
///
/// See [`create_context()`](crate::create_context()) for usage.
pub fn use_context<T>(context: &'static LocalKey<Context<T>>) -> Rc<T> {
  context.with(|context| {
    let ptr = react_bindings::use_rust_context(context.as_ref()) as *const T;
    let value = unsafe {
      let value = ManuallyDrop::new(Rc::from_raw(ptr));
      (*value).clone()
    };

    value
  })
}
