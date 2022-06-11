use crate::{react_bindings, VNode};
use std::{any::type_name, rc::Rc};
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

  /// Returns a [`VNode`] to be included in a render function.
  fn build(self) -> VNode
  where
    Self: Sized,
  {
    self.build_with_key(None)
  }

  /// Returns a [`VNode`] to be included in a render function with the given
  /// [React key].
  ///
  /// [React key]: https://reactjs.org/docs/lists-and-keys.html
  fn build_with_key(self, key: Option<&str>) -> VNode
  where
    Self: Sized,
  {
    VNode(react_bindings::create_rust_component(
      // This does not uniquely identify the component, but it is good enough
      type_name::<Self>(),
      &key.into(),
      &ComponentWrapper(Box::new(self)).into(),
    ))
  }
}

impl<T: Component> Component for Rc<T> {
  fn render(&self) -> VNode {
    <T as Component>::render(&self)
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
