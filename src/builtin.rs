use crate::{component::JsComponent, react_bindings};
use wasm_bindgen::JsValue;

#[derive(Debug, Default, Clone)]
pub struct Fragment(JsValue);

impl Fragment {
  pub fn new() -> JsComponent<Fragment> {
    JsComponent::new(Fragment(react_bindings::FRAGMENT.with(JsValue::clone)))
  }
}

impl AsRef<JsValue> for Fragment {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}

#[derive(Debug, Default, Clone)]
pub struct StrictMode(JsValue);

impl StrictMode {
  pub fn new() -> JsComponent<StrictMode> {
    JsComponent::new(StrictMode(
      react_bindings::STRICT_MODE.with(JsValue::clone),
    ))
  }
}

impl AsRef<JsValue> for StrictMode {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}
