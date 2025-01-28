use super::Component;
use crate::{react_bindings::create_element, VNode};
use js_sys::{JsString, Object, Reflect};
use std::marker::PhantomData;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

#[derive(Debug, Clone)]
pub struct JsComponent<T>
where
  T: AsRef<JsValue>,
{
  typ: T,
  props: Object,
  phantom: PhantomData<T>,
}

impl<T> Component for JsComponent<T>
where
  T: AsRef<JsValue> + 'static,
{
  fn render(&self, _children: VNode) -> impl Into<VNode> {
    VNode::empty()
  }

  fn extra_props(&self) -> Object {
    self.props.clone()
  }

  fn build(self, extra_props: &Object) -> VNode {
    VNode(create_element(self.typ.as_ref(), extra_props))
  }
}

impl<T> JsComponent<T>
where
  T: AsRef<JsValue>,
{
  pub fn new(typ: T) -> Self {
    Self {
      typ,
      props: Object::new(),
      phantom: PhantomData,
    }
  }

  pub fn prop(
    self,
    name: impl Into<JsString>,
    value: impl Into<JsValue>,
  ) -> Self {
    Reflect::set(&self.props, &name.into(), &value.into()).unwrap_throw();
    self
  }
}
