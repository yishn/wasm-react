use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

#[derive(Debug, Default, Clone)]
pub struct Style<'a> {
  data: Vec<(&'a str, JsValue)>,
}

impl<'a> Style<'a> {
  pub fn new() -> Style<'a> {
    Style { data: vec![] }
  }

  pub fn insert(mut self, key: &'a str, value: impl Into<JsValue>) -> Self {
    self.data.push((key, value.into()));
    self
  }
}

impl<'a> From<Style<'a>> for JsValue {
  fn from(style: Style<'a>) -> Self {
    let style_object = Object::new();

    for (name, value) in style.data.into_iter() {
      Reflect::set(&style_object, &name.into(), &value).unwrap();
    }

    style_object.into()
  }
}

impl<'a> From<Style<'a>> for (&'static str, JsValue) {
  fn from(style: Style<'a>) -> Self {
    ("style", style.into())
  }
}

pub fn style<'a>() -> Style<'a> {
  Style::new()
}
