use crate::{attr::Attr, react_bindings, VNode};
use wasm_bindgen::prelude::*;

/// Implement this trait on a struct to create a component with the struct as
/// props. The props will be completely controlled by Rust and will not be
/// exposed to Javascript. The props struct has to be `'static`.
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
///     html("div", None, ["Counter: ".into(), self.0.into()])
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
  /// **Do not** use this method in another render function. Instead, use one
  /// of the [`Component::into_vnode()`] methods.
  fn render(&self) -> VNode;

  /// Returns a [`VNode`] of the component so it can be included in a
  /// [`Component::render()`] function.
  fn into_vnode(self) -> VNode
  where
    Self: Sized + 'static,
  {
    VNode(react_bindings::create_component(
      Self::name(),
      Attr::new()
        .insert("component", ComponentWrapper(Box::new(self)))
        .into(),
    ))
  }

  /// Returns a [`VNode`] of the component with a [React key][key] so it can be
  /// included in a [`Component::render()`] function.
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  fn into_vnode_with_key(self, key: &str) -> VNode
  where
    Self: Sized + 'static,
  {
    VNode(react_bindings::create_component(
      Self::name(),
      Attr::new()
        .insert("key", key)
        .insert("component", ComponentWrapper(Box::new(self)))
        .into(),
    ))
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(Box<dyn Component>);

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen]
  pub fn render(props: &ComponentWrapper) -> JsValue {
    props.0.render().into()
  }
}
