use std::any::Any;
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

use crate::react_bindings;

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_TmpRef)]
pub struct TmpRef(Box<dyn Any>);

/// Temporarily persists a value.
/// 
/// The value will live until the next rerender. Callback functions will be 
/// persisted this way.
pub(crate) fn use_tmp_ref<T>(value: T, mut callback: impl FnMut(&T))
where
  T: 'static,
{
  react_bindings::use_rust_tmp_ref(
    TmpRef(Box::new(value)).into(),
    &mut |tmp_ref| callback(tmp_ref.0.downcast_ref().unwrap_throw()),
  )
}
