use crate::{
  c, create_element,
  hooks::{use_effect, use_ref, Deps},
  props::Props,
  react_bindings, Component, Context, VNode, VNodeList,
};
use js_sys::Reflect;
use std::{rc::Rc, thread::LocalKey};
use wasm_bindgen::UnwrapThrowExt;

/// Can be used to create a [React fragment][fragment].
///
/// [fragment]: https://reactjs.org/docs/fragments.html
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// # fn f() -> VNode {
/// Fragment.build(c![
///   h!(h1).build(c!["Hello World!"]),
///   h!(div).build(c!["No wrapper element"]),
/// ])
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Fragment;

impl Fragment {
  /// Returns a [`VNode`] which represents a [React fragment][fragment].
  ///
  /// [fragment]: https://reactjs.org/docs/fragments.html
  pub fn build(&self, children: VNodeList) -> VNode {
    create_element(&react_bindings::FRAGMENT, Props::new().as_ref(), children)
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
    "WasmReactContextProvider"
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
