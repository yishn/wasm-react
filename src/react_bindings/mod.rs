use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/src/react_bindings/mod.js")]
extern "C" {
  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue);

  #[wasm_bindgen(js_name = createElement)]
  pub fn create_element(
    typ: &JsValue,
    props: &JsValue,
    children: &JsValue,
  ) -> JsValue;
}
