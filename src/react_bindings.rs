use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/react-bindings.js")]
extern "C" {
  // wasm-react functions:

  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue);

  #[wasm_bindgen(js_name = createRustComponent)]
  pub fn create_rust_component(name: &str, props: &JsValue) -> JsValue;

  #[wasm_bindgen(js_name = createBuiltinComponent)]
  pub fn create_builtin_component(
    name: &str,
    props: &JsValue,
    children: &JsValue,
  ) -> JsValue;

  #[wasm_bindgen(js_name = useRustState)]
  pub fn use_rust_state() -> Function;

  #[wasm_bindgen(js_name = useRustRef)]
  pub fn use_rust_ref(create: &JsValue, on_free: &JsValue) -> usize;

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
