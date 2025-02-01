use super::Component;
use crate::{Prop, VNode};
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

#[derive(Debug, Clone, Copy)]
pub struct WithKey<C> {
  pub(super) component: C,
  pub(super) key: Prop<JsValue>,
}

impl<C> PartialEq for WithKey<C>
where
  C: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.component == other.component
  }
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static KEY: JsString = "key";
}

impl<C> Component for WithKey<C>
where
  C: Component,
{
  fn render(self, children: VNode) -> impl Into<VNode> {
    self.component.render(children)
  }

  fn name() -> &'static str {
    C::name()
  }

  fn props(self) -> Object {
    let props = self.component.props();

    KEY.with(|key| Reflect::set(&props, &key, &self.key.get()).unwrap_throw());

    props
  }

  fn build(self, props: &Object) -> VNode {
    self.component.build(props)
  }
}
