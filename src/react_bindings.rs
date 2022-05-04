use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/react-bindings.js")]
extern "C" {
  #[wasm_bindgen(js_name = setReact)]
  pub fn set_react(value: JsValue);

  #[wasm_bindgen(js_name = registerComponent)]
  pub fn register_component(name: &str);

  #[wasm_bindgen(js_name = getComponent)]
  pub fn get_component(name: &str) -> JsValue;

  #[wasm_bindgen(js_name = createComponent)]
  pub fn create_component(name: &str, props: JsValue) -> JsValue;

  #[wasm_bindgen(js_name = useRustState)]
  pub fn use_rust_state(create: &dyn Fn() -> usize, on_free: JsValue) -> Array;

  #[wasm_bindgen(js_name = cast)]
  pub fn cast_into_usize(value: JsValue) -> usize;

  #[wasm_bindgen(js_namespace = React, js_name = createElement)]
  pub fn create_element(
    name: &JsValue,
    props: &JsValue,
    children: &Array,
  ) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useEffect)]
  pub fn use_effect(f: JsValue, deps: JsValue);
}
