use crate::{react, VNode};
use wasm_bindgen::{convert::FromWasmAbi, JsValue};

pub trait Component: Sized + HasJsComponent {
  fn render(props: Self) -> VNode;

  fn into_vnode(self) -> VNode {
    VNode(react::create_component(
      Self::js_name(),
      <Self as HasJsComponent>::JsComponent::from(self).into(),
    ))
  }
}

pub trait HasJsComponent: Sized {
  type JsComponent: FromWasmAbi + Into<JsValue> + From<Self> + Into<Self>;

  fn js_name() -> &'static str;
}
