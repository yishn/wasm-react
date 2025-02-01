use super::Component;
use crate::{react_bindings::create_element, Prop, VNode};
use js_sys::{JsString, Object, Reflect};
use std::fmt::Debug;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub struct JsComponent<T> {
  typ: Prop<T>,
  props: Prop<Object>,
}

impl<T: 'static> JsComponent<T> {
  pub fn new(typ: T) -> Self {
    Self {
      typ: typ.into(),
      props: Object::new().into(),
    }
  }

  pub fn prop(
    self,
    name: impl Into<JsString>,
    value: impl Into<JsValue>,
  ) -> Self {
    Reflect::set(&self.props.get(), &name.into(), &value.into()).unwrap_throw();
    self
  }
}

impl<T> Debug for JsComponent<T>
where
  T: Debug + 'static,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("JsComponent")
      .field("typ", &self.typ)
      .field("props", &self.props)
      .finish()
  }
}

impl<T> Clone for JsComponent<T> {
  fn clone(&self) -> Self {
    Self {
      typ: self.typ.clone(),
      props: self.props.clone(),
    }
  }
}

impl<T> Copy for JsComponent<T> {}

impl<T> Component for JsComponent<T>
where
  T: AsRef<JsValue> + 'static,
{
  fn render(self, _children: VNode) -> impl Into<VNode> {
    ()
  }

  fn props(self) -> Object {
    self.props.get().clone()
  }

  fn build(self, props: &Object) -> VNode {
    VNode(create_element(self.typ.get().as_ref(), props))
  }
}
