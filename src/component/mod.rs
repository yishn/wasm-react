mod js_component;
mod with_children;
mod with_key;

pub use js_component::*;
pub use with_children::*;
pub use with_key::*;

use crate::{react_bindings::create_rust_component, VNode};
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
  fn render(&self, children: VNode) -> VNode;

  fn js_render(props: &JsValue) -> Result<JsValue, JsValue>
  where
    Self: TryFrom<JsValue, Error = JsValue>,
  {
    let component = Self::try_from(props.clone())?;
    let children = CHILDREN.with(|children| Reflect::get(props, children))?;

    Ok(component.render(VNode(children)).into())
  }

  fn extra_props(&self) -> Object {
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

  fn build_with_extra_props(self, extra_props: &Object) -> VNode {
    let name = type_name::<Self>();
    let component = ComponentWrapper::from(self);

    VNode(create_rust_component(name, component, extra_props))
  }

  fn build(self) -> VNode {
    let extra_props = self.extra_props();

    self.build_with_extra_props(&extra_props)
  }
}

#[doc(hidden)]
pub trait DynComponent: 'static {
  fn render(&self, children: VNode) -> VNode;
}

impl<T: Component> DynComponent for T {
  fn render(&self, children: VNode) -> VNode {
    Component::render(self, children)
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
