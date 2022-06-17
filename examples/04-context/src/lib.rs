use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_react::{
  c, create_context, export_components, h, hooks::use_state, Component,
  Context, ContextProvider, VNode,
};

pub enum Theme {
  LightMode,
  DarkMode,
}

thread_local! {
  pub static THEME_CONTEXT: Context<Theme>
    = create_context(Theme::LightMode.into());
}

pub struct App;

impl Component for App {
  fn render(&self) -> VNode {
    let theme = use_state(|| Rc::new(Theme::LightMode));
    let theme_class = match **theme.value() {
      Theme::LightMode => "light",
      Theme::DarkMode => "dark",
    };

    h!(div[.{theme_class}]).build(c![
      //
      ContextProvider::from(&THEME_CONTEXT)
        .value(Some({
          let value = theme.value();
          value.clone()
        }))
        .children(c![])
        .build()
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
