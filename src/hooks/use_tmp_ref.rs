use std::any::Any;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::react_bindings;

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_TmpRef)]
pub struct TmpRef(Box<dyn Any>);

pub fn use_tmp_ref<T>(value: T)
where
  T: 'static,
{
  react_bindings::use_rust_tmp_ref(TmpRef(Box::new(value)).into())
}
