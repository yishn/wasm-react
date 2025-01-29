use crate::hooks::{AnyRefContainer, OwnerContainer};
use crate::ComponentWrapper;
use js_sys::Function;
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

  #[wasm_bindgen(js_name = useOwnerSetup)]
  pub fn use_owner_setup(init: &dyn Fn() -> OwnerContainer);

  #[wasm_bindgen(js_name = getOwner)]
  pub fn get_owner(callback: &mut dyn FnMut(&OwnerContainer));

  #[wasm_bindgen(js_name = useRef)]
  pub fn use_ref(
    init: &dyn Fn() -> AnyRefContainer,
    callback: &mut dyn FnMut(&AnyRefContainer),
  );

  #[wasm_bindgen(js_name = useUpdate)]
  pub fn use_update() -> Function;
}
