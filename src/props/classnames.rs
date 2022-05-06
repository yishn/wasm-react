/// A trait for types to be used in [`classnames!`].
pub trait Classnames<'a> {
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

/// Constructs a [`String`] based on various types that implement [`Classnames`].
///
/// # Example
///
/// ```
/// classnames!["button", "blue"]
/// // Yields "button blue "
///
/// classnames!["button", false.then(|| "blue"), true.then(|| "disabled")]
/// // Yields "button disabled "
/// ```
#[macro_export]
macro_rules! classnames {
  [$( $impl_classnames:expr ),* $(,)?] => {
    {
      let mut result = String::new();
      $(
        $crate::props::Classnames::append_to(&$impl_classnames, &mut result);
      )*
      result
    }
  };
}
