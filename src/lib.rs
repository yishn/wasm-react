mod component;
mod react_bindings;
mod vnode;

pub use component::*;
pub use vnode::*;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Contains all functions exported to JS by `wasm-react`. These functions should
/// be called from JS only.
#[doc(hidden)]
#[wasm_bindgen]
pub struct WasmReact;

#[wasm_bindgen]
impl WasmReact {
  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue) {
    react_bindings::use_react(value);
  }

  #[wasm_bindgen(js_name = useRuntime)]
  pub fn use_runtime(value: &JsValue) {
    react_bindings::use_runtime(value);
  }
}
