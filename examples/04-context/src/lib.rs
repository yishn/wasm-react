mod button;
mod card;

use button::Button;
use card::Card;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_react::{
  c, create_context, export_components, h,
  hooks::{use_callback, use_state, Deps},
  Component, Context, ContextProvider, VNode,
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

    let handle_toggle_theme = use_callback(
      {
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
      },
      Deps::none(),
    );

    h!(div[.{theme_class}]).build(c![
      //
      ContextProvider::from(&THEME_CONTEXT)
        .value(Some({
          let value = theme.value();
          value.clone()
        }))
        .children(c![
          h!(p).build(c![
            //
            h!(label).build(c![
              h!(input)
                .html_type("checkbox")
                .checked(match **theme.value() {
                  Theme::LightMode => false,
                  Theme::DarkMode => true,
                })
                .on_change(&handle_toggle_theme)
                .build(c![]),
              "Dark Mode"
            ]),
          ]),
          //
          Card::new()
            .children(c![
              h!(p).build(c!["Hello World!"]),
              h!(p).build(c![
                Button::new().text("OK").build(),
                " ",
                Button::new().text("Cancel").build()
              ])
            ])
            .build()
        ])
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
