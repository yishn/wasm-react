use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/react-bindings.js")]
extern "C" {
  // wasm-react functions:

  #[wasm_bindgen(js_name = setReact)]
  pub fn set_react(value: &JsValue);

  #[wasm_bindgen(js_name = createRustComponent)]
  pub fn create_rust_component(name: &str, props: &JsValue) -> JsValue;

  #[wasm_bindgen(js_name = createBuiltinComponent)]
  pub fn create_builtin_component(
    name: &str,
    props: &JsValue,
    children: &JsValue,
  ) -> JsValue;

  #[wasm_bindgen(js_name = useRustState)]
  pub fn use_rust_state(default_value: &JsValue, on_free: &JsValue) -> Array;

  #[wasm_bindgen(js_name = cast)]
  pub fn cast_into_usize(value: &JsValue) -> usize;

  // From React namespace:

  #[wasm_bindgen(js_namespace = React, js_name = createElement)]
  pub fn create_element(
    name: &JsValue,
    props: &JsValue,
    children: &JsValue,
  ) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useEffect)]
  pub fn use_effect(f: &JsValue, deps: &JsValue);

  #[wasm_bindgen(js_namespace = React, js_name = useCallback)]
  pub fn use_callback(f: &JsValue, deps: &JsValue) -> Function;
}
