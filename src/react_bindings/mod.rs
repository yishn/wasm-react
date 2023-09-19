use crate::{hooks::RefContainerValue, ComponentWrapper, MemoComponentWrapper};
use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/react_bindings/react-bindings.js")]
extern "C" {
  #[wasm_bindgen(js_name = useReact)]
  pub fn use_react(value: &JsValue);

  #[wasm_bindgen(js_name = createElement)]
  pub fn create_element(
    typ: &JsValue,
    props: &JsValue,
    children: &JsValue,
  ) -> JsValue;

  #[wasm_bindgen(js_name = createRustComponent)]
  pub fn create_rust_component(
    name: &str,
    key: &JsValue,
    component: ComponentWrapper,
  ) -> JsValue;

  #[wasm_bindgen(js_name = createRustMemoComponent)]
  pub fn create_rust_memo_component(
    name: &str,
    key: &JsValue,
    component: MemoComponentWrapper,
  ) -> JsValue;

  #[wasm_bindgen(js_name = useRustRef)]
  pub fn use_rust_ref(
    create: &JsValue,
    callback: &mut dyn FnMut(&RefContainerValue),
  );

  #[wasm_bindgen(js_name = useRustTmpRef)]
  pub fn use_rust_tmp_ref(value: JsValue);

  #[wasm_bindgen(js_name = useRustState)]
  pub fn use_rust_state() -> Function;

  #[wasm_bindgen(js_name = useRustEffect)]
  pub fn use_rust_effect(effect: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = useRustLayoutEffect)]
  pub fn use_rust_layout_effect(effect: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = useRustContext)]
  pub fn use_rust_context(
    context: &JsValue,
    callback: &mut dyn FnMut(&RefContainerValue),
  );

  // From the React namespace:

  #[wasm_bindgen(js_namespace = React, js_name = Fragment)]
  pub static FRAGMENT: JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = Suspense)]
  pub static SUSPENSE: JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useRef)]
  pub fn use_ref(init: &JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = useId)]
  pub fn use_id() -> String;

  #[wasm_bindgen(js_namespace = React, js_name = useDeferredValue)]
  pub fn use_deferred_value(value: u8) -> u8;

  #[wasm_bindgen(js_namespace = React, js_name = useTransition)]
  pub fn use_transition() -> Array;

  #[wasm_bindgen(js_namespace = React, js_name = createContext)]
  pub fn create_context(value: RefContainerValue) -> JsValue;
}
