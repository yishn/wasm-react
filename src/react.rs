use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../react-bindings.js")]
extern "C" {
  #[wasm_bindgen(js_namespace = React, js_name = createElement)]
  pub fn create_element(
    name: JsValue,
    props: JsValue,
    children: Array,
  ) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useState)]
  pub fn use_state(value: &dyn Fn() -> JsValue) -> Array;

  #[wasm_bindgen(js_name = registerComponent)]
  pub fn register_component(name: &str);

  #[wasm_bindgen(js_name = getComponent)]
  pub fn get_component(name: &str) -> JsValue;
}
