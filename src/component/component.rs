use super::{KeyType, Memo, WithChildren, WithKey};
use crate::{
  hooks::{get_tmp_owner, OwnerContainer},
  react_bindings, VNode,
};
use js_sys::{JsString, Object, Reflect};
use std::any::{type_name, Any};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHILDREN: JsString = "children";
}

pub trait Component
where
  Self: Sized + Copy + 'static,
{
  fn render(self, children: VNode) -> impl Into<VNode>;

  fn name() -> &'static str {
    type_name::<Self>()
  }

  #[doc(hidden)]
  fn js_render(props: JsValue) -> Result<JsValue, JsValue>
  where
    Self: TryFrom<JsValue, Error = JsValue>,
  {
    let component = Self::try_from(props)?;
    let props = component.props();

    CHILDREN.with(|children| {
      Reflect::set(&props, children, &Reflect::get(&props, children)?)
    })?;

    Ok(component.build(&props).into())
  }

  fn props(self) -> Object {
    Object::new()
  }

  fn key(self, key: impl KeyType) -> WithKey<Self> {
    let owner = get_tmp_owner();

    WithKey {
      component: self,
      key: owner.insert(key.into()),
    }
  }

  fn children(self, children: impl Into<VNode>) -> WithChildren<Self> {
    let owner = get_tmp_owner();

    WithChildren {
      component: self,
      children: owner.insert(children.into()),
    }
  }

  fn memoized(self) -> Memo<Self>
  where
    Self: PartialEq,
  {
    Memo(self)
  }

  fn build(self, props: &Object) -> VNode {
    let name = Self::name();
    let component = ComponentWrapper::from(self);

    VNode(react_bindings::create_rust_component(
      name, component, props,
    ))
  }
}

impl<T> From<T> for VNode
where
  T: Component,
{
  fn from(component: T) -> Self {
    let props = component.props();
    component.build(&props)
  }
}

#[doc(hidden)]
pub trait DynComponent: 'static {
  fn render(&self, children: VNode) -> VNode;
}

impl<T> DynComponent for T
where
  T: Component,
{
  fn render(&self, children: VNode) -> VNode {
    Component::render(*self, children).into()
  }
}

#[doc(hidden)]
pub trait DynMemoComponent: DynComponent {
  fn as_any(&self) -> &dyn Any;

  fn eq(&self, other: &Box<dyn DynMemoComponent>) -> bool;
}

impl<T> DynMemoComponent for T
where
  T: Component + PartialEq,
{
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn eq(&self, other: &Box<dyn DynMemoComponent>) -> bool {
    other
      .as_any()
      .downcast_ref()
      .map(|other| T::eq(&self, other))
      .unwrap_or_default()
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(Box<dyn DynComponent>);

impl ComponentWrapper {
  pub fn from(component: impl DynComponent) -> Self {
    Self(Box::new(component))
  }
}

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen(js_name = newOwner)]
  pub fn new_owner(&self) -> OwnerContainer {
    OwnerContainer::new()
  }

  pub fn render(&self, children: JsValue) -> JsValue {
    self.0.render(VNode(children)).into()
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_MemoComponentWrapper)]
pub struct MemoComponentWrapper(Box<dyn DynMemoComponent>);

impl MemoComponentWrapper {
  pub fn from(component: impl DynMemoComponent) -> Self {
    Self(Box::new(component))
  }
}

#[wasm_bindgen(js_class = __WasmReact_MemoComponentWrapper)]
impl MemoComponentWrapper {
  #[wasm_bindgen(js_name = newOwner)]
  pub fn new_owner(&self) -> OwnerContainer {
    OwnerContainer::new()
  }

  pub fn render(&self, children: JsValue) -> JsValue {
    self.0.render(VNode(children)).into()
  }

  pub fn eq(&self, other: &MemoComponentWrapper) -> bool {
    self.0.eq(&other.0)
  }
}
