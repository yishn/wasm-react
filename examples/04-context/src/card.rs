use crate::{Theme, THEME_CONTEXT};
use wasm_react::{h, hooks::use_context, props::Style, Component, VNode};

#[derive(Debug, Default)]
pub struct Card {
  children: VNode,
}

impl Card {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn build(mut self, children: impl Into<VNode>) -> VNode {
    self.children = children.into();
    Component::build(self)
  }
}

impl Component for Card {
  fn render(&self) -> VNode {
    let theme = use_context(&THEME_CONTEXT);
    let style = {
      let mut style =
        Style::new().padding("1px 10px").border("1px solid black");

      if let Theme::DarkMode = *theme {
        style = style
          .background_color("#333")
          .color("#eee")
          .border_color("#ccc");
      }

      style
    };

    h!(div[."card"]).style(&style).build(self.children.clone())
  }
}
