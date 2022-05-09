use crate::VNode;
use wasm_bindgen::prelude::*;

/// Implement this trait on a struct to create a component with the struct as
/// props.
///
/// The props will be completely controlled by Rust, which makes rendering them
/// relatively simple in Rust. However, since the props struct cannot be
/// constructed in JS, these components cannot be exposed to JS. This means only
/// components written in Rust can render a `Component` by default.
///
/// See [`export_component!`] for how to expose components for JS consumption.
///
/// # Example
///
/// ```
/// # use wasm_react::*;
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
pub trait Component: 'static {
  /// The name of the component that will be displayed in the React Developer
  /// Tools, should be the struct name.
  fn name() -> &'static str
  where
    Self: Sized;

  /// The render function.
  ///
  /// **Do not** call this method in another render function. Instead, use the
  /// [`children!`](crate::children) macro to include your component.
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
  pub fn render(&self) -> JsValue {
    self.0.render().into()
  }
}

/// This macro can be used to expose your [`Component`] for JS consumption via
/// `wasm-bindgen`.
///
/// Requirement is that you implement the [`TryFrom<JsValue, Error = JsValue>`](core::convert::TryFrom)
/// trait on your component and that you do not export anything else that has
/// the same name as your component.
///
/// Therefore, it is only recommended to use this macro if you're writing a
/// library for JS consumption only, or if you're writing a standalone
/// application, since this will pollute the export namespace, which isn't
/// desirable if you're writing a library for Rust consumption only.
///
/// # Example
///
/// Implement [`TryFrom<JsValue, Error = JsValue>`](core::convert::TryFrom) on
/// your component and export it:
///
/// ```
/// # use wasm_react::*;
/// # use wasm_bindgen::prelude::*;
/// # use js_sys::Reflect;
/// #
/// pub struct Counter {
///   counter: i32,
/// }
///
/// impl Component for Counter {
///   # fn name() -> &'static str { "" }
///   # fn render(&self) -> VNode { VNode::empty() }
///   /* ... */
/// }
///
/// impl TryFrom<JsValue> for Counter {
///   type Error = JsValue;
///
///   fn try_from(value: JsValue) -> Result<Self, Self::Error> {
///     let diff = Reflect::get(&value, &"counter".into())?
///       .as_f64()
///       .ok_or(JsError::new("`counter` property not found"))?;
///
///     Ok(Counter { counter: diff as i32 })
///   }
/// }
///
/// export_component!(Counter);
/// ```
///
/// In JS, you can use it like any other component:
///
/// ```js
/// import React from "react";
/// import init, { Counter } from "./path/to/wasm-bindings.js";
///
/// function SomeOtherJsComponent(props) {
///   return (
///     <div>
///       <Counter counter={0} />
///     </div>
///   );
/// }
/// ```
#[macro_export]
macro_rules! export_component {
  ($component:ident) => {
    use wasm_bindgen::JsValue;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[wasm_bindgen]
    pub fn $component(props: JsValue) -> Result<JsValue, JsValue>
    where
      $component: $crate::Component + TryFrom<JsValue, Error = JsValue>,
    {
      let component = $component::try_from(props)?;
      Ok(component.render().into())
    }
  };
}
