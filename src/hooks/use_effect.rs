use crate::{react_bindings, Callback};
use js_sys::Array;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum Deps {
  All,
  Some(Array),
}

impl Deps {
  pub fn push(&self, value: impl Into<JsValue>) {
    match self {
      Deps::All => (),
      Deps::Some(arr) => {
        arr.push(&value.into());
      }
    }
  }
}

impl From<Deps> for JsValue {
  fn from(value: Deps) -> Self {
    match value {
      Deps::All => JsValue::undefined(),
      Deps::Some(deps) => deps.iter().collect::<Array>().into(),
    }
  }
}

impl Default for Deps {
  fn default() -> Self {
    Deps::All
  }
}

pub fn use_effect<F, G>(f: F, deps: Deps)
where
  F: Fn() -> G + 'static,
  G: Fn() + 'static,
{
  react_bindings::use_effect(
    Callback::new(move |_: JsValue| -> JsValue {
      let g = f();
      Callback::new(move |_: JsValue| g()).into()
    })
    .into(),
    deps.into(),
  )
}

#[macro_export]
macro_rules! deps {
  [*] => { $crate::hooks::Deps::All };
  [$( $into_js:expr ),* $(,)?] => {
    {
      let deps = $crate::hooks::Deps::Some(js_sys::Array::new());
      $( deps.push($into_js); )*
      deps
    }
  }
}
