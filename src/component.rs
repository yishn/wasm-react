use crate::{react_bindings, VNode};
use std::{
  any::{type_name, Any},
  rc::Rc,
};
use wasm_bindgen::prelude::*;

/// Implement this trait on a struct to create a component with the struct as
/// props.
///
/// The props will be completely controlled by Rust, which makes rendering them
/// relatively simple in Rust. However, since the props struct cannot be
/// constructed in JS, these components cannot be exposed to JS. This means only
/// components written in Rust can render a `Component` by default.
///
/// See [`export_components!`](crate::export_components!) for how to expose
/// components for JS consumption.
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// struct Counter(i32);
///
/// impl Component for Counter {
///   fn render(&self) -> VNode {
///     h!(div).build(c!["Counter: ", self.0])
///   }
/// }
/// ```
pub trait Component: 'static {
  /// The render function.
  ///
  /// **Do not** call this method in another render function. Instead, use
  /// [`Component::build()`] to include your component.
  fn render(&self) -> VNode;

  fn key(self, key: Option<&str>) -> Keyed<Self>
  where
    Self: Sized,
  {
    Keyed(self, key)
  }

  /// Returns a [`VNode`] to be included in a render function.
  fn build(self) -> VNode
  where
    Self: Sized,
  {
    VNode(react_bindings::create_rust_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<Self>(),
      None,
      ComponentWrapper(Box::new(self)),
    ))
  }

  /// Returns a memoized version of your component that skips rendering if props
  /// haven't changed.
  ///
  /// If your component renders the same result given the same props, you can
  /// memoize your component for a performance boost.
  ///
  /// You have to implement [`PartialEq`] on your [`Component`] for this to work.
  ///
  /// # Example
  ///
  /// ```
  /// # use std::rc::Rc;
  /// # use wasm_react::*;
  /// #[derive(PartialEq)]
  /// struct MessageBox {
  ///   message: Rc<str>,
  /// }
  ///
  /// impl Component for MessageBox {
  ///   fn render(&self) -> VNode {
  ///     h!(h1[."message-box"]).build(c![&*self.message])
  ///   }
  /// }
  ///
  /// struct App;
  ///
  /// impl Component for App {
  ///   fn render(&self) -> VNode {
  ///     h!(div[#"app"]).build(c![
  ///       MessageBox {
  ///         message: Rc::from("Hello World!"),
  ///       }
  ///       .memoized()
  ///       .build()
  ///     ])
  ///   }
  /// }
  /// ```
  fn memoized(self) -> Memoized<Self>
  where
    Self: Sized + PartialEq,
  {
    Memoized(self)
  }
}

impl<T: Component> Component for Rc<T> {
  fn render(&self) -> VNode {
    <T as Component>::render(&self)
  }
}

pub struct Keyed<'a, T>(pub(crate) T, pub(crate) Option<&'a str>);

impl<T: Component> Keyed<'_, T> {
  /// Returns a [`VNode`] to be included in a render function.
  pub fn build(self) -> VNode {
    VNode(react_bindings::create_rust_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<Self>(),
      self.1,
      ComponentWrapper(Box::new(self.0)),
    ))
  }
}

impl<T: Component + PartialEq> Keyed<'_, Memoized<T>> {
  /// Returns a [`VNode`] to be included in a render function.
  pub fn build(self) -> VNode {
    VNode(react_bindings::create_rust_memo_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<T>(),
      self.1,
      MemoComponentWrapper(Box::new(self.0 .0)),
    ))
  }
}

/// Wraps your component to let React skip rendering if props haven't changed.
///
/// See [`Component::memoized()`].
pub struct Memoized<T>(pub(crate) T);

impl<T: Component + PartialEq> Memoized<T> {
  pub fn key(self, key: Option<&str>) -> Keyed<Self> {
    Keyed(self, key)
  }

  /// Returns a [`VNode`] to be included in a render function.
  pub fn build(self) -> VNode {
    VNode(react_bindings::create_rust_memo_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<T>(),
      None,
      MemoComponentWrapper(Box::new(self.0)),
    ))
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(pub(crate) Box<dyn Component>);

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen]
  pub fn render(&self) -> JsValue {
    self.0.render().into()
  }
}

pub(crate) trait MemoComponent: Component {
  fn as_any(&self) -> &dyn Any;
  fn eq(&self, other: &dyn Any) -> bool;
}

impl<T: Component + PartialEq> MemoComponent for T {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn eq(&self, other: &dyn Any) -> bool {
    other
      .downcast_ref::<T>()
      .map(|other| PartialEq::eq(self, other))
      .unwrap_or(false)
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_MemoComponentWrapper)]
pub struct MemoComponentWrapper(pub(crate) Box<dyn MemoComponent>);

#[wasm_bindgen(js_class = __WasmReact_MemoComponentWrapper)]
impl MemoComponentWrapper {
  #[wasm_bindgen]
  pub fn render(&self) -> JsValue {
    self.0.render().into()
  }

  #[wasm_bindgen]
  pub fn eq(&self, other: &MemoComponentWrapper) -> bool {
    self.0.eq(other.0.as_any())
  }
}
