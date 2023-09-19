use crate::{Theme, THEME_CONTEXT};
use std::rc::Rc;
use wasm_react::{h, hooks::use_context, props::Style, Component, VNode};

#[derive(Debug)]
pub struct Button {
  text: Rc<str>,
}

impl Button {
  pub fn new() -> Self {
    Self {
      text: Rc::from("Button"),
    }
  }

  pub fn text(mut self, text: &str) -> Self {
    self.text = Rc::from(text);
    self
  }
}

impl Component for Button {
  fn render(&self) -> VNode {
    let theme = use_context(&THEME_CONTEXT);
    let style = {
      let mut style =
        Style::new().padding("5px 10px").border("1px solid black");

      if let Theme::DarkMode = *theme {
        style = style
          .background_color("#444")
          .color("#eee")
          .border_color("#ccc");
      }

      style
    };

    h!(button).style(&style).build(&*self.text)
  }
}
