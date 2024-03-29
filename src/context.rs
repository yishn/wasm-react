use crate::{
  create_element, hooks::RefContainerValue, props::Props, react_bindings,
  Component, VNode,
};
use js_sys::Reflect;
use std::{marker::PhantomData, rc::Rc, thread::LocalKey};
use wasm_bindgen::{intern, JsValue, UnwrapThrowExt};

/// Represents a [React context][context] that can hold a global state.
///
/// See [`create_context()`] for usage.
///
/// [context]: https://react.dev/learn/passing-data-deeply-with-context
#[derive(Debug)]
pub struct Context<T> {
  js_context: JsValue,
  phantom: PhantomData<T>,
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
/// [context]: https://react.dev/learn/passing-data-deeply-with-context
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*, props::*};
/// # pub enum Theme { DarkMode, LightMode }
/// #
/// thread_local! {
///   // Pass in a default value for the context.
///   static THEME_CONTEXT: Context<Theme> = create_context(Theme::LightMode.into());
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
///       .value(Some(Theme::DarkMode.into()))
///       .build(
///         Toolbar.build(),
///       )
///   }
/// }
///
/// struct Toolbar;
///
/// impl Component for Toolbar {
///   fn render(&self) -> VNode {
///     // Theme context does not have to be passed down explicitly.
///     h!(div).build(Button.build())
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
///         &Style::new()
///           .background_color(match *theme {
///             Theme::LightMode => "white",
///             Theme::DarkMode => "black",
///           })
///       )
///       .build("Button")
///   }
/// }
/// ```
pub fn create_context<T: 'static>(init: Rc<T>) -> Context<T> {
  Context {
    js_context: react_bindings::create_context(RefContainerValue(init)),
    phantom: PhantomData,
  }
}

/// A component that can make the given context available for its subtrees.
///
/// See [`create_context()`] for usage.
#[derive(Debug, Clone)]
pub struct ContextProvider<T: 'static> {
  context: &'static LocalKey<Context<T>>,
  value: Option<Rc<T>>,
  children: VNode,
}

impl<T: 'static> ContextProvider<T> {
  /// Creates a new [`ContextProvider`] from the given context.
  pub fn from(context: &'static LocalKey<Context<T>>) -> Self {
    Self {
      context,
      value: None,
      children: ().into(),
    }
  }

  /// Sets the value of the context to be passed down.
  pub fn value(mut self, value: Option<Rc<T>>) -> Self {
    self.value = value;
    self
  }

  /// Returns a [`VNode`] to be included in a render function.
  pub fn build(mut self, children: impl Into<VNode>) -> VNode {
    self.children = children.into();
    Component::build(self)
  }
}

impl<T: 'static> Component for ContextProvider<T> {
  fn render(&self) -> VNode {
    self.context.with(|context| {
      create_element(
        &Reflect::get(context.as_ref(), &intern("Provider").into())
          .expect_throw("cannot read from context object"),
        &{
          let mut props = Props::new();

          if let Some(value) = self.value.as_ref() {
            props = props.insert(
              intern("value"),
              &RefContainerValue(value.clone()).into(),
            );
          }

          props
        },
        self.children.clone(),
      )
    })
  }
}
