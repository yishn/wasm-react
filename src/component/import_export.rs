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
        #[::wasm_bindgen::prelude::wasm_bindgen(thread_local_v2, js_name = $Component)]
        static [<__WASMREACT_IMPORT_ $Name:upper>]: ::wasm_bindgen::JsValue;
      }

      $( #[$meta] )*
      #[derive(Debug, Clone)]
      $vis struct $Name(::wasm_bindgen::JsValue);

      impl $Name {
        #[doc = "Returns an `JsComponent<" $Name ">` struct that provides "
                "convenience methods for adding props."]
        pub fn new() -> $crate::component::JsComponent<$Name> {
          $crate::component::JsComponent::new(
            $Name(
              [<__WASMREACT_IMPORT_ $Name:upper>]
                .with(::wasm_bindgen::JsValue::clone)
            )
          )
        }
      }

      impl AsRef<::wasm_bindgen::JsValue> for $Name {
        fn as_ref(&self) -> &::wasm_bindgen::JsValue {
          &self.0
        }
      }
    }

    $( $crate::import_components! { #[$from] $( $tail )* } )?
  };
}

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
        <$Component as $crate::Component>::js_render(props)
      }
    }

    $( $crate::export_components! { $( $tail )* } )?
  };
}
