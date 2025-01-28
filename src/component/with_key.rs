use super::Component;
use crate::VNode;
use js_sys::{JsString, Object, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, UnwrapThrowExt};

pub trait KeyType: Into<JsValue> {}

macro_rules! impl_key_type {
  { $( $T:ty ),* $( , )? } => {
    $( impl KeyType for $T {} )*
  };
}

impl_key_type! {
  &str, String, JsString,
  f32, f64,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WithKey<C: Component> {
  pub(super) component: C,
  pub(super) key: JsValue,
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static KEY: JsString = "key";
}

impl<C: Component> Component for WithKey<C> {
  fn render(&self, children: VNode) -> impl Into<VNode> {
    self.component.render(children)
  }

  fn props(&self) -> Object {
    let props = self.component.props();

    KEY.with(|key| Reflect::set(&props, &key, &self.key).unwrap_throw());

    props
  }

  fn build(self, props: &Object) -> VNode {
    self.component.build(props)
  }
}
