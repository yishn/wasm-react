use js_sys::Reflect;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_react::{c, export_components, h, Component, VNode};

pub struct App {
  name: Option<Rc<str>>,
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(App {
      name: Reflect::get(&value, &"name".into())?
        .as_string()
        .map(|x| x.into_boxed_str().into()),
    })
  }
}

impl Component for App {
  fn render(&self) -> VNode {
    h!(h1).build(c![
      //
      if let Some(name) = self.name.as_ref() {
        format!("Hello {name}!")
      } else {
        "Hello World!".to_string()
      }
    ])
  }
}

export_components! { App }
