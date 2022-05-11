use js_sys::{Function, Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/react_bindings/react-bindings.js")]
extern "C" {
  #[wasm_bindgen(js_name = useReact, catch)]
  pub fn use_react(value: &JsValue) -> Result<(), JsValue>;

  #[wasm_bindgen(js_name = getRustComponent)]
  pub fn get_rust_component(name: &str) -> Function;

  #[wasm_bindgen(js_name = createRustComponent)]
  pub fn create_rust_component(name: &str, props: &JsValue) -> JsValue;

  #[wasm_bindgen(js_name = useRustRef)]
  pub fn use_rust_ref(create: &JsValue, handler: &JsValue) -> JsValue;

  #[wasm_bindgen(js_name = useRustState)]
  pub fn use_rust_state() -> Function;

  #[wasm_bindgen(js_name = useRustEffect)]
  pub fn use_rust_effect(effect: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = useRustLayoutEffect)]
  pub fn use_rust_layout_effect(effect: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = childrenToArray, catch)]
  pub fn children_to_array(children: &JsValue) -> Result<Array, JsValue>;

  #[wasm_bindgen(js_name = cast)]
  pub fn cast_to_usize(value: &JsValue) -> usize;

  // From the React namespace:

  #[wasm_bindgen(js_namespace = React, js_name = Fragment)]
  pub static FRAGMENT: JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = createElement)]
  pub fn create_element(
    name: &JsValue,
    props: &JsValue,
    children: &JsValue,
  ) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = createContext)]
  pub fn create_context(value: &JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useContext)]
  pub fn use_context(context: &JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useCallback)]
  pub fn use_callback(f: &JsValue, deps: &JsValue) -> Function;

  #[wasm_bindgen(js_namespace = React, js_name = useRef)]
  pub fn use_ref(init: &JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useId)]
  pub fn use_id() -> String;

  #[wasm_bindgen(js_namespace = React, js_name = useDeferredValue)]
  pub fn use_deferred_value(value: u8) -> u8;

  #[wasm_bindgen(js_namespace = React, js_name = useTransition)]
  pub fn use_transition() -> Array;
}
