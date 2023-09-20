mod button;
mod card;

use button::Button;
use card::Card;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_react::{
  create_context, export_components, h, hooks::use_state, Component, Context,
  ContextProvider, VNode, callback::Callback,
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

    let result = h!(div[.{theme_class}]).build(
      //
      ContextProvider::from(&THEME_CONTEXT)
        .value(Some({
          let value = theme.value();
          value.clone()
        }))
        .build((
          h!(p).build((
            //
            h!(label).build((
              h!(input)
                .html_type("checkbox")
                .checked(match **theme.value() {
                  Theme::LightMode => false,
                  Theme::DarkMode => true,
                })
                .on_change(&Callback::new({
                  let mut theme = theme.clone();

                  move |_| {
                    theme.set(|theme| {
                      match *theme {
                        Theme::LightMode => Theme::DarkMode,
                        Theme::DarkMode => Theme::LightMode,
                      }
                      .into()
                    })
                  }
                }))
                .build(()),
              "Dark Mode",
            )),
          )),
          //
          Card::new()
            .children((
              h!(p).build("Hello World!"),
              h!(p).build((
                Button::new().text("OK").build(),
                " ",
                Button::new().text("Cancel").build(),
              )),
            ))
            .build(),
        )),
    );
    result
  }
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

export_components! { App }
