use crate::VNode;
use wasm_bindgen::prelude::*;

/// Implement this trait on a struct to create a component with the struct as
/// props. The props struct has to be `'static`.
///
/// The props will be completely controlled by Rust, which makes rendering them
/// relatively simple in Rust. However, since the props struct cannot be
/// constructed in JS, these components cannot be exposed to JS. This means only
/// components written in Rust can render a `Component`.
///
/// # Example
///
/// ```
/// struct Counter(i32);
///
/// impl Component for Counter {
///   fn name() -> &'static str {
///     "Counter"
///   }
///
///   fn render(&self) -> VNode {
///     h!(div).build(children!["Counter: ", self.0])
///   }
/// }
/// ```
pub trait Component {
  /// The name of the component that will be displayed in the React Developer
  /// Tools, usually the struct name. It has to be unique among all components
  /// defined in Rust.
  fn name() -> &'static str
  where
    Self: Sized + 'static;

  /// The render function.
  ///
  /// **Do not** call this method in another render function.
  fn render(&self) -> VNode;

  /// Override this method to provide a [React key][key] when rendering.
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  fn key(&self) -> Option<String> {
    None
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(pub(crate) Box<dyn Component>);

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen]
  pub fn render(props: &ComponentWrapper) -> JsValue {
    props.0.render().into()
  }
}
