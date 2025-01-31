mod callback;
mod component;
mod dom;
pub mod hooks;
mod prop;
mod react_bindings;
mod vnode;

pub use callback::*;
pub use component::*;
pub use dom::*;
#[doc(hidden)]
pub use paste::paste;
pub use prop::*;
pub use vnode::*;

use wasm_bindgen::{
  prelude::{wasm_bindgen, Closure},
  JsValue,
};

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

pub fn start_transition(f: impl FnMut() + 'static) {
  react_bindings::start_transition(&Closure::new(f).into_js_value());
}
