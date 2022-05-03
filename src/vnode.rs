use std::fmt::Display;
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  describe::WasmDescribe,
  JsValue,
};

#[derive(Clone)]
pub struct VNode(pub(crate) JsValue);

impl From<VNode> for JsValue {
  fn from(value: VNode) -> Self {
    value.0
  }
}

impl<T> From<T> for VNode
where
  T: Display + Into<JsValue>,
{
  fn from(value: T) -> Self {
    VNode(value.into())
  }
}

impl WasmDescribe for VNode {
  fn describe() {
    JsValue::describe()
  }
}

impl IntoWasmAbi for VNode {
  type Abi = <JsValue as IntoWasmAbi>::Abi;

  fn into_abi(self) -> Self::Abi {
    self.0.into_abi()
  }
}

impl FromWasmAbi for VNode {
  type Abi = <JsValue as FromWasmAbi>::Abi;

  unsafe fn from_abi(js: Self::Abi) -> Self {
    VNode(JsValue::from_abi(js))
  }
}
