use crate::{
  c, create_element,
  hooks::{use_effect, use_ref, Deps},
  props::Props,
  react_bindings, Component, VNode, VNodeList,
};
use js_sys::Reflect;
use std::{rc::Rc, thread::LocalKey};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

/// Represents a [React context][context] that can hold a global state.
///
/// See [`create_context()`] for usage.
///
/// [context]: https://reactjs.org/docs/context.html
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

/// Creates a new [React context][context] that can hold a global state.
///
/// Use [`ContextProvider`] to make the context available for its subtrees and
/// [`use_context()`](crate::hooks::use_context()) to get access to the context
/// value.
///
/// [context]: https://reactjs.org/docs/context.html
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*, props::*};
/// # pub enum Theme { DarkMode, LightMode }
/// #
/// thread_local! {
///   // Pass in a default value for the context.
///   static THEME_CONTEXT: Context<Theme> = create_context(Theme::LightMode);
/// }
///
/// struct App;
///
/// impl Component for App {
///   fn render(&self) -> VNode {
///     // Use a `ContextProvider` to pass the context value to the trees below.
///     // In this example, we are passing down `Theme::DarkMode`.
///
///     ContextProvider::from(&THEME_CONTEXT)
///       .value(Theme::DarkMode)
///       .build(c![Toolbar.build()])
///   }
/// }
///
/// struct Toolbar;
///
/// impl Component for Toolbar {
///   fn render(&self) -> VNode {
///     // Theme context does not have to be passed down explicitly.
///     h!(div).build(c![Button.build()])
///   }
/// }
///
/// struct Button;
///
/// impl Component for Button {
///   fn render(&self) -> VNode {
///     // Use the `use_context` hook to get access to the context value.
///     let theme = use_context(&THEME_CONTEXT);
///
///     h!(button)
///       .style(
///         Style::new()
///           .background_color(match *theme {
///             Theme::LightMode => "white",
///             Theme::DarkMode => "black",
///           })
///       )
///       .build(c![])
///   }
/// }
/// ```
pub fn create_context<T: 'static>(init: T) -> Context<T> {
  Context {
    fallback_value: Rc::new(init),
    js_context: react_bindings::create_context(&JsValue::undefined()),
  }
}

/// A component that can make the given context available for its subtrees.
///
/// See [`create_context()`] for usage.
#[derive(Debug, Clone)]
pub struct ContextProvider<T: 'static> {
  context: &'static LocalKey<Context<T>>,
  value: Option<Rc<T>>,
  children: VNodeList,
}

impl<T: 'static> ContextProvider<T> {
  /// Creates a new `ContextProvider` from the given context.
  pub fn from(context: &'static LocalKey<Context<T>>) -> Self {
    Self {
      context,
      value: None,
      children: c![],
    }
  }

  /// Sets the value of the context to be passed down.
  pub fn value(mut self, value: T) -> Self {
    self.value = Some(Rc::new(value));
    self
  }

  /// Returns a [`VNode`] to be included in the render function of a component.
  pub fn build(mut self, children: VNodeList) -> VNode {
    self.children = children;
    Component::build(self)
  }

  /// Returns a [`VNode`] to be included in the render function of a component
  /// with the given [React key].
  ///
  /// [React key]: https://reactjs.org/docs/lists-and-keys.html
  pub fn build_with_key(
    mut self,
    key: Option<&str>,
    children: VNodeList,
  ) -> VNode {
    self.children = children;
    Component::build_with_key(self, key)
  }
}

impl<T: 'static> Component for ContextProvider<T> {
  fn render(&self) -> VNode {
    self.context.with(|context| {
      let value = self
        .value
        .clone()
        .unwrap_or_else(|| context.fallback_value.clone());
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
        &Reflect::get(context.as_ref(), &"Provider".into())
          .expect_throw("cannot read from context object"),
        &Props::new().insert("value", value_ref.as_ref()),
        self.children.clone(),
      )
    })
  }
}
