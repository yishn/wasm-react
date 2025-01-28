use crate::JsComponent;
use js_sys::JsString;
use wasm_bindgen::JsValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Tag(JsString);

impl AsRef<JsValue> for Tag {
  fn as_ref(&self) -> &JsValue {
    self.0.as_ref()
  }
}

impl Tag {
  pub fn new(typ: impl Into<JsString>) -> JsComponent<Tag> {
    JsComponent::new(Tag(typ.into()))
  }
}
