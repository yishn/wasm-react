use crate::{react_bindings, VNode};
use std::{
  any::{type_name, Any},
  rc::Rc,
};
use wasm_bindgen::prelude::*;

#[doc(hidden)]
pub struct BuildParams<T> {
  name: &'static str,
  key: Option<Rc<str>>,
  create_component:
    Box<dyn FnOnce(&'static str, Option<Rc<str>>, T) -> JsValue>,
}

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
pub trait Component: Sized + 'static {
  /// The render function.
  ///
  /// **Do not** call this method in another render function. Instead, use
  /// [`Component::build()`] to include your component.
  fn render(&self) -> VNode;

  /// Sets the [React key][key].
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  fn key(self, key: Option<&str>) -> Keyed<Self> {
    Keyed(self, key.map(Rc::from))
  }

  #[doc(hidden)]
  fn build_params(&self) -> BuildParams<Self> {
    BuildParams {
      name: type_name::<Self>(),
      key: None,
      create_component: Box::new(|name, key, component| {
        react_bindings::create_rust_component(
          name,
          key.as_ref().map(|x| &**x),
          ComponentWrapper(Box::new(component)),
        )
      }),
    }
  }

  /// Returns a [`VNode`] to be included in a render function.
  fn build(self) -> VNode {
    let BuildParams {
      name,
      key,
      create_component,
    } = self.build_params();

    VNode(create_component(name, key, self))
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
    Self: PartialEq,
  {
    Memoized(self)
  }
}

impl<T: Component> Component for Rc<T> {
  fn render(&self) -> VNode {
    <T as Component>::render(&self)
  }
}

/// Wraps your component to assign a [React key][key] to it.
///
/// See [`Component::key()`].
///
/// [key]: https://reactjs.org/docs/lists-and-keys.html
#[derive(Debug, PartialEq)]
pub struct Keyed<T>(T, Option<Rc<str>>);

impl<T: Component> Component for Keyed<T> {
  fn render(&self) -> VNode {
    self.0.render()
  }

  fn build_params(&self) -> BuildParams<Self> {
    let BuildParams {
      name,
      create_component,
      ..
    } = self.0.build_params();

    BuildParams {
      name,
      key: self.1.clone(),
      create_component: Box::new(|name, key, component| {
        create_component(name, key, component.0)
      }),
    }
  }
}

/// Wraps your component to let React skip rendering if props haven't changed.
///
/// See [`Component::memoized()`].
#[derive(Debug, PartialEq)]
pub struct Memoized<T>(T);

impl<T: Component + PartialEq> Component for Memoized<T> {
  fn render(&self) -> VNode {
    self.0.render()
  }

  fn build_params(&self) -> BuildParams<Self> {
    let BuildParams { name, key, .. } = self.0.build_params();

    BuildParams {
      name,
      key,
      create_component: Box::new(|name, key, component| {
        react_bindings::create_rust_memo_component(
          name,
          key.as_ref().map(|x| &**x),
          MemoComponentWrapper(Box::new(component.0)),
        )
      }),
    }
  }
}

trait ObjectSafeComponent {
  fn render(&self) -> VNode;
}

impl<T: Component> ObjectSafeComponent for T {
  fn render(&self) -> VNode {
    Component::render(self)
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(Box<dyn ObjectSafeComponent>);

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen]
  pub fn render(&self) -> JsValue {
    self.0.render().into()
  }
}

trait ObjectSafeMemoComponent: ObjectSafeComponent {
  fn as_any(&self) -> &dyn Any;
  fn eq(&self, other: &dyn Any) -> bool;
}

impl<T: Component + PartialEq> ObjectSafeMemoComponent for T {
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
pub struct MemoComponentWrapper(Box<dyn ObjectSafeMemoComponent>);

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
