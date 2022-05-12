use crate::VNode;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::prelude::*;

/// Implement this trait on a struct to create a component with the struct as
/// props.
///
/// The props will be completely controlled by Rust, which makes rendering them
/// relatively simple in Rust. However, since the props struct cannot be
/// constructed in JS, these components cannot be exposed to JS. This means only
/// components written in Rust can render a `Component` by default.
///
/// See [`export_component!`](crate::export_component!) for how to expose
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
  /// **Do not** call this method in another render function. Instead, use the
  /// [`c!`](crate::c!) macro to include your component.
  fn render(&self) -> VNode;

  /// Override this method to provide a [React key][key] when rendering.
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  fn key(&self) -> Option<String> {
    None
  }
}

impl<T: Component> Component for Rc<T> {
  fn render(&self) -> VNode {
    self.deref().render()
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
