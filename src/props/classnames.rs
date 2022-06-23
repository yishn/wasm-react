/// A trait for types to be used in [`classnames!`](crate::classnames!).
pub trait Classnames {
  /// Appends the class to a string.
  fn append_to(&self, string: &mut String);
}

impl Classnames for &str {
  fn append_to(&self, string: &mut String) {
    string.push_str(self);
    string.push(' ');
  }
}

impl Classnames for String {
  fn append_to(&self, string: &mut String) {
    (&self[..]).append_to(string);
  }
}

impl Classnames for &String {
  fn append_to(&self, string: &mut String) {
    (&self[..]).append_to(string);
  }
}

impl<T: Classnames> Classnames for Option<T> {
  fn append_to(&self, string: &mut String) {
    if let Some(value) = self {
      value.append_to(string);
    }
  }
}
