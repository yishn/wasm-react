use crate::{
  c, create_element,
  hooks::{use_effect, use_ref, Deps},
  props::Props,
  react_bindings, Component, VNode, VNodeList,
};
use js_sys::Reflect;
use std::{rc::Rc, thread::LocalKey};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

#[derive(Debug)]
pub struct Context<T> {
  fallback_value: Rc<T>,
  js_context: JsValue,
}

impl<T> AsRef<JsValue> for Context<T> {
  fn as_ref(&self) -> &JsValue {
    &self.js_context
  }
}

impl<T> From<Context<T>> for JsValue {
  fn from(value: Context<T>) -> Self {
    value.js_context
  }
}

pub fn create_context<T: 'static>(init: T) -> Context<T> {
  Context {
    fallback_value: Rc::new(init),
    js_context: react_bindings::create_context(&JsValue::undefined()),
  }
}

pub struct ContextProvider<T: 'static> {
  context: &'static LocalKey<Context<T>>,
  value: Option<Rc<T>>,
  children: VNodeList,
}

impl<T: 'static> ContextProvider<T> {
  pub fn from(context: &'static LocalKey<Context<T>>) -> Self {
    Self {
      context,
      value: None,
      children: c![],
    }
  }

  pub fn value(mut self, value: T) -> Self {
    self.value = Some(Rc::new(value));
    self
  }

  pub fn build(mut self, children: VNodeList) -> VNode {
    self.children = children;
    self.into()
  }
}

impl<T: 'static> Component for ContextProvider<T> {
  fn name() -> &'static str {
    "WasmReact.ContextProvider"
  }

  fn render(&self) -> VNode {
    let value = self.value.clone().unwrap_or_else(|| {
      self.context.with(|context| context.fallback_value.clone())
    });
    let value_ref = use_ref(value.clone());

    use_effect(
      {
        let mut value_ref = value_ref.clone();

        move || {
          value_ref.set_current(value);
          || ()
        }
      },
      Deps::all(),
    );

    create_element(
      &Reflect::get(
        &self.context.with(|context| context.js_context.clone()),
        &"Provider".into(),
      )
      .unwrap_throw(),
      Props::new().insert("value", value_ref.as_ref()),
      self.children.clone(),
    )
  }
}
