use wasm_bindgen::prelude::*;
use wasm_react::{
  c, export_components, import_components,
  props::{Props, Style},
  Component,
};

import_components! {
  #[wasm_bindgen(module = "/js/mui-components.js")]
  AppBar, Toolbar, Typography, IconButton, Button, Box as BoxComponent,
  Container, Card, CardContent, CardActions, MenuIcon
}

pub struct App;

impl Component for App {
  fn render(&self) -> wasm_react::VNode {
    BoxComponent(&Props::new()).build(c![
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
        ]),
      ]),
      //
      Container(
        &Props::new().insert(
          "sx",
          &Style::new()
            .margin_top(8)
            .padding_top(2)
            .padding_bottom(2)
            .as_ref()
        )
      )
      .build(c![
        //
        Card(
          &Props::new()
            .insert("variant", &"outlined".into())
            .insert("sx", Style::new().max_width(345).as_ref())
        )
        .build(c![
          //
          CardContent(&Props::new()).build(c![
            //
            Typography(
              &Props::new()
                .insert("variant", &"h5".into())
                .insert("component", &"h2".into())
                .insert("sx", &Style::new().margin_bottom(1.5).as_ref())
            )
            .build(c!["Hello World!"]),
            Typography(
              //
              &Props::new().insert("variant", &"body2".into())
            )
            .build(c![
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit, ",
              "sed do eiusmod tempor incididunt ut labore et dolore magna ",
              "aliqua. Ut enim ad minim veniam, quis nostrud exercitation ",
              "ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis ",
              "aute irure dolor in reprehenderit in voluptate velit esse ",
              "cillum dolore eu fugiat nulla pariatur. Excepteur sint ",
              "occaecat cupidatat non proident, sunt in culpa qui officia ",
              "deserunt mollit anim id est laborum."
            ])
          ]),
          CardActions(&Props::new()).build(c![
            //
            Button(&Props::new().insert("size", &"small".into()))
              .build(c!["Learn More"])
          ])
        ])
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
