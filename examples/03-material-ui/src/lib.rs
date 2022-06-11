use wasm_bindgen::prelude::*;
use wasm_react::{
  c, export_components, h, import_components,
  props::{Props, Style},
  Component,
};

import_components! {
  #[wasm_bindgen(module = "/js/mui-components.js")]
  AppBar, Toolbar, Typography, IconButton, Button, Box, MenuIcon
}

pub struct App;

impl Component for App {
  fn render(&self) -> wasm_react::VNode {
    h!(div).build(c![
      //
      AppBar(&Props::new()).build(c![
        //
        Toolbar(&Props::new()).build(c![
          IconButton(
            &Props::new()
              .insert("color", &"inherit".into())
              .insert("edge", &"start".into())
              .insert("sx", Style::new().margin_right(2).as_ref())
          )
          .build(c![MenuIcon(&Props::new()).build(c![])]),
          //
          Typography(
            &Props::new()
              .insert("variant", &"h6".into())
              .insert("color", &"inherit".into())
              .insert("component", &"h1".into())
              .insert("sx", Style::new().flex_grow(1).as_ref())
          )
          .build(c!["MUI Example Application"]),
          //
          Button(&Props::new().insert("color", &"inherit".into()))
            .build(c!["Hello World!"])
        ]),
      ]),
    ])
  }
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

export_components! { App }
