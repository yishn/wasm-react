use crate::component::JsComponent;
use js_sys::JsString;
use wasm_bindgen::JsValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Tag(JsString);

impl Tag {
  pub fn new(typ: impl Into<JsString>) -> JsComponent<Tag> {
    JsComponent::new(Tag(typ.into()))
  }
}

impl AsRef<JsValue> for Tag {
  fn as_ref(&self) -> &JsValue {
    self.0.as_ref()
  }
}

#[macro_export]
macro_rules! h {
  (@internal $tag:block $( #$id:literal )? $( .$( $classnames:tt )+ )?) => {
    $tag
    $( .id($id) )?
    $( .classname($crate::classnames!(.$( $classnames )+)) )?
  };
  ($tag:literal $( $tt:tt )*) => {
    $crate::h!(@internal { $crate::Tag::new($tag) } $( $tt )*)
  };
  ($tag:ident $( $tt:tt )*) => {
    $crate::h!(@internal
      { $crate::Tag::new(::wasm_bindgen::intern(stringify!($tag))) }
      $( $tt )*
    )
  };
}
