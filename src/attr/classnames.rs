use std::borrow::Cow;
use wasm_bindgen::JsValue;

pub trait Classnames<'a> {
  fn to_classnames(&self) -> Cow<'a, str>;
}

impl<'a> Classnames<'a> for &'a str {
  fn to_classnames(&self) -> Cow<'a, str> {
    (*self).into()
  }
}

impl<'a, T: Classnames<'a>> Classnames<'a> for (T, bool) {
  fn to_classnames(&self) -> Cow<'a, str> {
    if self.1 {
      self.0.to_classnames()
    } else {
      Cow::Borrowed("")
    }
  }
}

impl<'a, T: Classnames<'a>> Classnames<'a> for Option<T> {
  fn to_classnames(&self) -> Cow<'a, str> {
    self
      .as_ref()
      .map(|x| x.to_classnames())
      .unwrap_or_else(|| Cow::Borrowed(""))
  }
}

impl<'a, 'b, T: Classnames<'a>> Classnames<'a> for &'b [T] {
  fn to_classnames(&self) -> Cow<'a, str> {
    self
      .iter()
      .map(|x| x.to_classnames())
      .reduce(|acc, x| {
        let mut string = acc.into_owned();
        string.push_str(" ");
        string.push_str(&x);

        Cow::Owned(string)
      })
      .unwrap_or_else(|| Cow::Borrowed(""))
  }
}

pub fn classnames<'a>(names: impl Classnames<'a>) -> (&'static str, JsValue) {
  ("className", names.to_classnames().into_owned().into())
}
