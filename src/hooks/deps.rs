use js_sys::Array;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum Deps {
  All,
  Some(Array),
}

impl Deps {
  pub fn push(&self, value: impl Into<JsValue>) {
    if let Deps::Some(arr) = self {
      arr.push(&value.into());
    }
  }
}

impl From<Deps> for JsValue {
  fn from(value: Deps) -> Self {
    match value {
      Deps::All => JsValue::undefined(),
      Deps::Some(deps) => deps.into(),
    }
  }
}

#[macro_export]
macro_rules! deps {
  (*) => { $crate::hooks::Deps::All };
  ($( $into_js:expr ),* $(,)?) => {
    {
      let deps = $crate::hooks::Deps::Some(js_sys::Array::new());
      $( deps.push($into_js); )*
      deps
    }
  }
}
