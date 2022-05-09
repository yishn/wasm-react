use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/react_bindings/react-bindings.js")]
extern "C" {
  #[wasm_bindgen(js_name = useReact, catch)]
  pub fn use_react(value: &JsValue) -> Result<(), JsValue>;

  #[wasm_bindgen(js_name = getRustComponent)]
  pub fn get_rust_component(name: &str) -> Function;

  #[wasm_bindgen(js_name = createRustComponent)]
  pub fn create_rust_component(name: &str, props: &JsValue) -> JsValue;

  #[wasm_bindgen(js_name = useRustState)]
  pub fn use_rust_state() -> Function;

  #[wasm_bindgen(js_name = useRustRef)]
  pub fn use_rust_ref(create: &JsValue, handler: &JsValue) -> JsValue;

  // From the React namespace:

  #[wasm_bindgen(js_namespace = React, js_name = Fragment)]
  pub static FRAGMENT: JsValue;

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

  #[wasm_bindgen(js_namespace = React, js_name = useRef)]
  pub fn use_ref(init: &JsValue) -> JsValue;
}
