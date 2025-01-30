use super::Component;
use crate::{hooks::get_tmp_owner, react_bindings::create_element, VNode};
use generational_box::GenerationalBox;
use js_sys::{JsString, Object, Reflect};
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub struct JsComponent<T> {
  typ: GenerationalBox<T>,
  props: GenerationalBox<Object>,
  phantom: PhantomData<T>,
}

impl<T: 'static> JsComponent<T> {
  pub fn new(typ: T) -> Self {
    let owner = get_tmp_owner();

    Self {
      typ: owner.insert(typ),
      props: owner.insert(Object::new()),
      phantom: PhantomData,
    }
  }

  pub fn prop(
    self,
    name: impl Into<JsString>,
    value: impl Into<JsValue>,
  ) -> Self {
    Reflect::set(&self.props.read(), &name.into(), &value.into())
      .unwrap_throw();
    self
  }
}

impl<T> Debug for JsComponent<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("JsComponent")
      .field("typ", &self.typ)
      .field("props", &self.props)
      .field("phantom", &self.phantom)
      .finish()
  }
}

impl<T> Clone for JsComponent<T> {
  fn clone(&self) -> Self {
    Self {
      typ: self.typ.clone(),
      props: self.props.clone(),
      phantom: self.phantom.clone(),
    }
  }
}

impl<T> Copy for JsComponent<T> {}

impl<T> Component for JsComponent<T>
where
  T: AsRef<JsValue> + 'static,
{
  fn render(self, _children: VNode) -> impl Into<VNode> {
    VNode::empty()
  }

  fn props(self) -> Object {
    self.props.read().clone()
  }

  fn build(self, props: &Object) -> VNode {
    VNode(create_element(self.typ.read().as_ref(), props))
  }
}
