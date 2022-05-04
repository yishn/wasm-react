use std::borrow::Cow;

/// A trait used with [`H::class_name()`](super::H::class_name).
pub trait Classnames<'a> {
  fn to_value(&self) -> Cow<'a, str>;
}

impl<'a> Classnames<'a> for &'a str {
  fn to_value(&self) -> Cow<'a, str> {
    (*self).into()
  }
}

impl<'a, T: Classnames<'a>> Classnames<'a> for (T, bool) {
  fn to_value(&self) -> Cow<'a, str> {
    if self.1 {
      self.0.to_value()
    } else {
      Cow::Borrowed("")
    }
  }
}

impl<'a, T: Classnames<'a>> Classnames<'a> for Option<T> {
  fn to_value(&self) -> Cow<'a, str> {
    self
      .as_ref()
      .map(|x| x.to_value())
      .unwrap_or_else(|| Cow::Borrowed(""))
  }
}

impl<'a, 'b, T: Classnames<'a>> Classnames<'a> for &'b [T] {
  fn to_value(&self) -> Cow<'a, str> {
    self
      .iter()
      .map(|x| x.to_value())
      .reduce(|acc, x| Cow::Owned(acc.into_owned() + " " + &x))
      .unwrap_or_else(|| Cow::Borrowed(""))
  }
}
