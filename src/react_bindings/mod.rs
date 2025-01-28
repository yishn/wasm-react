use crate::ComponentWrapper;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/src/react_bindings/mod.js")]
extern "C" {
  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue);

  #[wasm_bindgen(js_name = useRuntime)]
  pub fn use_runtime(value: &JsValue);

  #[wasm_bindgen(js_name = createElement)]
  pub fn create_element(typ: &JsValue, props: &JsValue) -> JsValue;

  #[wasm_bindgen(js_name = createRustComponent)]
  pub fn create_rust_component(
    name: &str,
    component: ComponentWrapper,
    extra_props: &JsValue,
  ) -> JsValue;
}
