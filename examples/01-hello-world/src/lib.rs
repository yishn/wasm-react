use js_sys::Reflect;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_react::{export_components, h, Component, VNode};

pub struct App {
  name: Option<Rc<str>>,
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(App {
      name: Reflect::get(&value, &"name".into())?
        .as_string()
        .map(|x| x.into()),
    })
  }
}

impl Component for App {
  fn render(&self) -> VNode {
    h!(h1).build(
      //
      if let Some(name) = self.name.as_ref() {
        format!("Hello {name}!")
      } else {
        "Hello World!".to_string()
      },
    )
  }
}

export_components! { App }
