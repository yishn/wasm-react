use crate::component::{ComponentWrapper, MemoComponentWrapper};
use crate::hooks::{AnyRefContainer, OwnerContainer};
use js_sys::{Function, JsString};
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

  #[wasm_bindgen(js_name = createRustMemoComponent)]
  pub fn create_rust_memo_component(
    original_name: &str,
    memo_name: &str,
    component: MemoComponentWrapper,
    extra_props: &JsValue,
  ) -> JsValue;

  #[wasm_bindgen(js_name = getOwner)]
  pub fn get_owner(callback: &mut dyn FnMut(&OwnerContainer));

  #[wasm_bindgen(js_name = getTmpOwner)]
  pub fn get_tmp_owner(callback: &mut dyn FnMut(&OwnerContainer));

  #[wasm_bindgen(js_name = useRef)]
  pub fn use_ref(
    init: &mut dyn FnMut() -> AnyRefContainer,
    callback: &mut dyn FnMut(&AnyRefContainer),
  );

  #[wasm_bindgen(js_name = useUpdate)]
  pub fn use_update() -> Function;

  #[wasm_bindgen(js_name = useEffect)]
  pub fn use_effect(f: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = useLayoutEffect)]
  pub fn use_layout_effect(f: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = useInsertionEffect)]
  pub fn use_insertion_effect(f: &JsValue, dep: u8);

  #[wasm_bindgen(js_name = useTransition)]
  pub fn use_transition(callback: &mut dyn FnMut(bool, Function));

  #[wasm_bindgen(js_name = memo)]
  pub fn memo(component: &JsValue, equal: &JsValue) -> JsValue;

  // From the React namespace:

  #[wasm_bindgen(thread_local_v2, js_namespace = React, js_name = Fragment)]
  pub static FRAGMENT: JsValue;

  #[wasm_bindgen(thread_local_v2, js_namespace = React, js_name = Profiler)]
  pub static PROFILER: JsValue;

  #[wasm_bindgen(thread_local_v2, js_namespace = React, js_name = StrictMode)]
  pub static STRICT_MODE: JsValue;

  #[wasm_bindgen(thread_local_v2, js_namespace = React, js_name = Suspense)]
  pub static SUSPENSE: JsValue;

  #[wasm_bindgen(js_namespace = React, js_name = startTransition)]
  pub fn start_transition(f: &JsValue);

  #[wasm_bindgen(js_namespace = React, js_name = useId)]
  pub fn use_id() -> JsString;

  #[wasm_bindgen(js_namespace = React, js_name = useDeferredValue)]
  pub fn use_deferred_value(value: u8, init: u8) -> u8;
}
