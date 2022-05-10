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
  context: Context<T>,
  value: Option<Rc<T>>,
  children: VNodeList,
}

impl<T: 'static> ContextProvider<T> {
  pub fn from(context: &'static LocalKey<Context<T>>) -> Self {
    Self {
      context: context.with(|context| context.clone()),
      value: None,
      children: c![],
    }
  }

  pub fn value(mut self, value: Option<T>) -> Self {
    self.value = value.map(|t| Rc::new(t));
    self
  }

  pub fn build(mut self, children: VNodeList) -> VNode {
    self.children = children;
    self.into()
  }
}

impl<T: 'static> Component for ContextProvider<T> {
  fn name() -> &'static str {
    "ContextProvider"
  }

  fn render(&self) -> VNode {
    let value = self
      .value
      .clone()
      .unwrap_or_else(|| self.context.fallback_value.clone());
    let mut value_ref = use_ref(value.clone());

    use_effect(
      || {
        value_ref.set_current(value);
        || ()
      },
      Deps::all(),
    );

    create_element(
      &Reflect::get(&self.context.js_context, &"Provider".into())
        .unwrap_throw(),
      Props::new().insert("value", value_ref.as_ref()),
      self.children.clone(),
    )
  }
}
