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
      ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>
      where
        $Component: $crate::Component
          + TryFrom<::wasm_bindgen::JsValue, Error = ::wasm_bindgen::JsValue>
      {
        <$Component as $crate::Component>::js_render(&props)
      }
    }

    $( $crate::export_components! { $( $tail )* } )?
  };
}
