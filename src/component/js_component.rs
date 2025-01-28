use super::Component;
use crate::{react_bindings::create_element, VNode};
use js_sys::{JsString, Object, Reflect};
use std::marker::PhantomData;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

#[derive(Debug, Clone)]
pub struct JsComponent<M = ()> {
  typ: JsValue,
  props: Object,
  phantom: PhantomData<M>,
}

impl<M: 'static> Component for JsComponent<M> {
  fn render(&self, _children: VNode) -> VNode {
    VNode::empty()
  }

  fn extra_props(&self) -> Object {
    self.props.clone()
  }

  fn build_with_extra_props(self, extra_props: &Object) -> VNode {
    VNode(create_element(&self.typ, extra_props))
  }
}

impl<M> JsComponent<M> {
  pub fn new(typ: JsValue) -> Self {
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
