use crate::{react, Callback};
use js_sys::Array;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum Deps {
  All,
  None,
  Some(Array),
}

impl Deps {
  pub fn push(self, value: impl Into<JsValue>) -> Self {
    match self {
      Deps::All => Deps::All,
      Deps::None => {
        Deps::Some(Some(value.into()).into_iter().collect::<Array>())
      }
      Deps::Some(arr) => {
        arr.push(&value.into());
        Deps::Some(arr)
      }
    }
  }
}

impl From<Deps> for JsValue {
  fn from(value: Deps) -> Self {
    match value {
      Deps::All => JsValue::undefined(),
      Deps::None => Array::new().into(),
      Deps::Some(deps) => deps.iter().collect::<Array>().into(),
    }
  }
}

impl Default for Deps {
  fn default() -> Self {
    Deps::None
  }
}

pub fn use_effect<G, F>(f: F, deps: Deps)
where
  G: Fn() + 'static,
  F: Fn() -> G + 'static,
{
  react::use_effect(
    Callback::new(move |_: JsValue| -> JsValue {
      let g = f();
      Callback::new(move |_: JsValue| g()).into()
    })
    .into(),
    deps.into(),
  )
}
