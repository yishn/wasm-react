use super::{KeyType, WithChildren, WithKey};
use crate::{
  hooks::{use_owner_setup, use_tmp_owner_setup},
  react_bindings::create_rust_component,
  VNode,
};
use js_sys::{JsString, Object, Reflect};
use std::any::type_name;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHILDREN: JsString = "children";
}

pub trait Component: Sized + 'static {
  fn render(&self, children: VNode) -> impl Into<VNode>;

  fn name() -> &'static str {
    type_name::<Self>()
  }

  fn prerender(&self) {
    use_owner_setup();
    use_tmp_owner_setup();
  }

  #[doc(hidden)]
  fn js_render(props: &JsValue) -> Result<JsValue, JsValue>
  where
    Self: TryFrom<JsValue, Error = JsValue>,
  {
    let component = Self::try_from(props.clone())?;
    let children = CHILDREN.with(|children| Reflect::get(props, children))?;

    Ok(DynComponent::render(&component, VNode(children)).into())
  }

  fn props(&self) -> Object {
    Object::new()
  }

  fn key(self, key: impl KeyType) -> WithKey<Self> {
    WithKey {
      component: self,
      key: key.into(),
    }
  }

  fn children(self, children: impl Into<VNode>) -> WithChildren<Self> {
    WithChildren {
      component: self,
      children: children.into(),
    }
  }

  fn build(self, props: &Object) -> VNode {
    let name = Self::name();
    let component = ComponentWrapper::from(self);

    VNode(create_rust_component(name, component, props))
  }
}

#[doc(hidden)]
pub trait DynComponent: 'static {
  fn render(&self, children: VNode) -> VNode;
}

impl<T: Component> DynComponent for T {
  fn render(&self, children: VNode) -> VNode {
    Component::prerender(self);
    Component::render(self, children).into()
  }
}

impl<T: Component> From<T> for VNode {
  fn from(component: T) -> Self {
    let extra_props = component.props();
    component.build(&extra_props)
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(Box<dyn DynComponent>);

impl ComponentWrapper {
  pub fn from(component: impl DynComponent) -> ComponentWrapper {
    ComponentWrapper(Box::new(component))
  }
}

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  pub fn render(&self, children: JsValue) -> JsValue {
    self.0.render(VNode(children)).into()
  }
}
