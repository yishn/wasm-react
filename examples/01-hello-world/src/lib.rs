use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_react::{
  c, export_components, h, import_components, props::Props, Component,
  Fragment, VNode,
};

import_components! {
  #[wasm_bindgen(module = "/src/myComponent.js")]
  MyComponent as MyJsComponent,
}

pub struct App {
  name: Option<String>,
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(App {
      name: Reflect::get(&value, &"name".into())?.as_string(),
    })
  }
}

impl Component for App {
  fn render(&self) -> VNode {
    Fragment.build(c![
      h!(h1).build(c![if let Some(name) = self.name.as_ref() {
        format!("Hello {}!", name)
      } else {
        "Hello World!".to_string()
      }]),
      //
      MyJsComponent(
        &Props::new().insert("text", &"This is an imported component".into())
      )
      .build(c![]),
    ])
  }
}

export_components! { App }
