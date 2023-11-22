/// A convenience macro to [`create_element()`](crate::create_element()) for
/// creating HTML element nodes.
///
/// Returns an [`H<HtmlTag>`](crate::props::H) struct that provides
/// auto-completion for HTML attributes and events.
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// # fn f() -> VNode {
/// h!(div)
///   .attr("id", &"app".into())
///   .build(
///     h!(h1).build("Hello World!")
///   )
/// # }
///
/// // <div id="app"><h1>Hello World!</h1></div>
///
/// # fn g() -> VNode {
/// h!("web-component")
///   .build("Hello World!")
/// # }
///
/// // <web-component>Hello World!</web-component>
/// ```
///
/// It is also possible to add an id and/or classes to the element using a terse
/// notation. You can use the same syntax as [`classnames!`](crate::classnames!).
///
/// ```
/// # use wasm_react::*;
/// # fn f() -> VNode {
/// h!(div[#"app"."some-class"."warning"])
///   .build("This is a warning!")
/// # }
///
/// // <div id="app" class="some-class warning">This is a warning!</div>
/// ```
#[macro_export]
macro_rules! h {
  ($tag:literal $( [$( #$id:literal )? $( .$( $classnames:tt )+ )?] )?) => {
    $crate::props::H::new($crate::props::HtmlTag($tag)) $(
      $( .id($id) )?
      $( .class_name(&$crate::classnames![.$( $classnames )+]) )?
    )?
  };
  ($tag:ident $( [$( #$id:literal )? $( .$( $classnames:tt )+ )?] )?) => {
    $crate::props::H::new($crate::props::HtmlTag(stringify!($tag))) $(
      $( .id($id) )?
      $( .class_name(&$crate::classnames![.$( $classnames )+]) )?
    )?
  };
}

/// A helper macro which can be used to clone a list of variables. Helpful for
/// creating a closure which clone-captures the environment.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// # fn f() {
/// let message = use_state(|| "Hello");
/// let counter = use_state(|| 0);
///
/// let cb = Callback::new({
///   clones!(message, mut counter);
///   move |delta: i32| {
///     println!("{}", message.value());
///     counter.set(|c| c + delta);
///   }
/// });
/// # }
/// ```
///
/// This is equivalent to the following:
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// # fn f() {
/// let message = use_state(|| "Hello");
/// let counter = use_state(|| 0);
///
/// let cb = Callback::new({
///   let message = message.clone();
///   let mut counter = counter.clone();
///
///   move |delta: i32| {
///     println!("{}", message.value());
///     counter.set(|c| c + delta);
///   }
/// });
/// # }
/// ```
#[macro_export]
macro_rules! clones {
  (@clones $(,)? mut $id:ident $( $tail:tt )*) => {
    let mut $id = $id.clone();
    $crate::clones!(@clones $( $tail )*);
  };
  (@clones $(,)? $id:ident $( $tail:tt )*) => {
    let $id = $id.clone();
    $crate::clones!(@clones $( $tail )*);
  };
  (@clones) => {};

  ($( $tt:tt )*) => {
    $crate::clones!(@clones $( $tt )*);
  };
}

/// Constructs a [`String`] based on various types that implement
/// [`Classnames`](crate::props::Classnames).
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// assert_eq!(
///   classnames![."button"."blue"],
///   "button blue ".to_string(),
/// );
///
/// let blue = false;
/// let disabled = true;
///
/// assert_eq!(
///   classnames![."button".blue.disabled],
///   "button disabled ".to_string(),
/// );
///
/// let is_blue = Some("blue");
/// let disabled = "disabled".to_string();
///
/// assert_eq!(
///   classnames![."button".{is_blue}.{&disabled}],
///   "button blue disabled ",
/// );
/// ```
#[macro_export]
macro_rules! classnames {
  [@single $result:ident <<] => {};

  // Handle string literals
  [@single $result:ident << .$str:literal $( $tail:tt )*] => {
    $crate::props::Classnames::append_to(&$str, &mut $result);
    $crate::classnames![@single $result << $( $tail ) *];
  };

  // Handle boolean variables
  [@single $result:ident << .$bool:ident $( $tail:tt )*] => {
    $crate::props::Classnames::append_to(
      &$bool.then(|| stringify!($bool)),
      &mut $result
    );
    $crate::classnames![@single $result << $( $tail ) *];
  };

  // Handle block expressions
  [@single $result:ident << .$block:block $( $tail:tt )*] => {
    $crate::props::Classnames::append_to(&$block, &mut $result);
    $crate::classnames![@single $result << $( $tail ) *];
  };

  [] => {
    ::std::string::String::new()
  };
  [$( $tt:tt )*] => {
    {
      let mut result = ::std::string::String::new();
      $crate::classnames![@single result << $( $tt )*];
      result
    }
  };
}

/// This macro can be used to expose your [`Component`](crate::Component) for JS
/// consumption via `wasm-bindgen`.
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
///   # fn render(&self) -> VNode { VNode::new() }
///   /* … */
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
/// export_components! { Counter }
/// ```
///
/// In JS, you can use it like any other component:
///
/// ```js
/// import React from "react";
/// import init, { Counter } from "./path/to/pkg/project.js";
///
/// function SomeOtherJsComponent(props) {
///   return (
///     <div>
///       <Counter counter={0} />
///     </div>
///   );
/// }
/// ```
///
/// You can export multiple components and also rename them:
///
/// ```
/// # use wasm_react::*;
/// # use wasm_bindgen::prelude::*;
/// # pub struct App; pub struct Counter;
/// # impl Component for App { fn render(&self) -> VNode { VNode::new() } }
/// # impl TryFrom<JsValue> for App {
/// #   type Error = JsValue;
/// #   fn try_from(_: JsValue) -> Result<Self, Self::Error> { todo!() }
/// # }
/// # impl Component for Counter { fn render(&self) -> VNode { VNode::new() } }
/// # impl TryFrom<JsValue> for Counter {
/// #   type Error = JsValue;
/// #   fn try_from(_: JsValue) -> Result<Self, Self::Error> { todo!() }
/// # }
/// export_components! {
///   /// Some doc comment for the exported component.
///   App as CounterApp,
///   Counter
/// }
/// ```
#[macro_export]
macro_rules! export_components {
  {} => {};
  {
    $( #[$meta:meta] )*
    $Component:ident $( , $( $tail:tt )* )?
  } => {
    $crate::export_components! {
      $( #[$meta] )*
      $Component as $Component $( , $( $tail )* )?
    }
  };
  {
    $( #[$meta:meta] )*
    $Component:ty as $Name:ident $( , $( $tail:tt )* )?
  } => {
    $crate::paste! {
      $( #[$meta] )*
      #[allow(non_snake_case)]
      #[allow(dead_code)]
      #[doc(hidden)]
      #[::wasm_bindgen::prelude::wasm_bindgen(js_name = $Name)]
      pub fn [<__WasmReact_Export_ $Name>](
        props: ::wasm_bindgen::JsValue,
      ) -> ::wasm_bindgen::JsValue
      where
        $Component: $crate::Component
          + TryFrom<::wasm_bindgen::JsValue, Error = ::wasm_bindgen::JsValue>
      {
        let component_ref = $crate::hooks::use_memo({
          let props = props.clone();

          move || $Component::try_from(props).unwrap()
        }, $crate::hooks::Deps::some(props));

        $crate::react_bindings::use_rust_tmp_refs();

        let component = component_ref.value();
        $crate::Component::render(&*component).into()
      }
    }

    $( $crate::export_components! { $( $tail )* } )?
  };
}

/// This macro can be used to import JS React components for Rust consumption
/// via `wasm-bindgen`.
///
/// Make sure that the components you import use the same React runtime as
/// specified for `wasm-react`.
///
/// # Example
///
/// Assume the JS components are defined and exported in `/.dummy/myComponents.js`:
///
/// ```js
/// import "https://unpkg.com/react/umd/react.production.min.js";
///
/// export function MyComponent(props) { /* … */ }
/// export function PublicComponent(props) { /* … */ }
/// export function RenamedComponent(props) { /* … */ }
/// ```
///
/// Then you can import them using `import_components!`:
///
/// ```
/// # use wasm_react::*;
/// # use wasm_bindgen::prelude::*;
/// import_components! {
///   #[wasm_bindgen(module = "/.dummy/myComponents.js")]
///
///   /// Some doc comment for the imported component.
///   MyComponent,
///   /// This imported component will be made public.
///   pub PublicComponent,
///   /// You can rename imported components.
///   RenamedComponent as pub OtherComponent,
/// }
/// ```
///
/// Now you can include the imported components in your render function:
///
/// ```
/// # use wasm_react::{*, props::*};
/// # use wasm_bindgen::prelude::*;
/// # import_components! { #[wasm_bindgen(inline_js = "")] MyComponent }
/// # struct App;
/// # impl Component for App {
/// fn render(&self) -> VNode {
///   h!(div).build(
///     MyComponent::new()
///       .attr("prop", &"Hello World!".into())
///       .build(())
///   )
/// }
/// # }
/// ```
///
/// # Defining Custom Convenience Methods
///
/// `MyComponent::new()` returns an [`H<MyComponent>`](crate::props::H) which
/// can be used to define convenience methods by using a new extension trait:
///
/// ```
/// # use wasm_react::{*, props::*};
/// # use wasm_bindgen::prelude::*;
/// # import_components! { #[wasm_bindgen(inline_js = "")] MyComponent }
/// trait HMyComponentExt {
///   fn prop(self, value: &str) -> Self;
/// }
///
/// impl HMyComponentExt for H<MyComponent> {
///   fn prop(self, value: &str) -> Self {
///     self.attr("prop", &value.into())
///   }
/// }
///
/// /* … */
///
/// # struct App;
/// # impl Component for App {
/// fn render(&self) -> VNode {
///   h!(div).build(
///     MyComponent::new()
///       .prop("Hello World!")
///       .build(())
///   )
/// }
/// # }
/// ```
#[macro_export]
macro_rules! import_components {
  { #[$from:meta] } => {};
  {
    #[$from:meta]
    $( #[$meta:meta] )*
    $vis:vis $Component:ident $( , $( $tail:tt )* )?
  } => {
    $crate::import_components! {
      #[$from]
      $( #[$meta] )*
      $Component as $vis $Component $( , $( $tail )* )?
    }
  };
  {
    #[$from:meta]
    $( #[$meta:meta] )*
    $Component:ident as $vis:vis $Name:ident $( , $( $tail:tt )* )?
  } => {
    $crate::paste! {
      #[$from]
      extern "C" {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = $Component)]
        static [<__WASMREACT_IMPORT_ $Name:upper>]: wasm_bindgen::JsValue;
      }

      $( #[$meta] )*
      #[derive(Debug, Clone, Copy)]
      $vis struct $Name;

      impl $Name {
        #[doc = "Returns an `H<" $Name ">` struct that provides convenience "
                "methods for adding props."]
        pub fn new() -> $crate::props::H<$Name> {
          $crate::props::H::new($Name)
        }
      }

      impl $crate::props::HType for $Name {
        fn as_js(&self) -> std::borrow::Cow<'_, JsValue> {
          std::borrow::Cow::Borrowed(&[<__WASMREACT_IMPORT_ $Name:upper>])
        }
      }
    }

    $( $crate::import_components! { #[$from] $( $tail )* } )?
  };
}
