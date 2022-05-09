/// A trait for types to be used in [`classnames!`](crate::classnames!).
pub trait Classnames<'a> {
  /// Appends the class to a string.
  fn append_to(&self, string: &mut String);
}

impl<'a> Classnames<'a> for &'a str {
  fn append_to(&self, string: &mut String) {
    string.push_str(self);
    string.push(' ');
  }
}

impl<'a> Classnames<'a> for String {
  fn append_to(&self, string: &mut String) {
    (&self[..]).append_to(string);
  }
}

impl<'a, T: Classnames<'a>> Classnames<'a> for Option<T> {
  fn append_to(&self, string: &mut String) {
    if let Some(value) = self {
      value.append_to(string);
    }
  }
}
