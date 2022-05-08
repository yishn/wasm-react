/// A trait for types to be used in [`classnames!`].
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

/// Constructs a [`String`] based on various types that implement [`Classnames`].
///
/// # Example
///
/// ```
/// # use wasm_react::*;
/// assert_eq!(
///   classnames![."button"."blue"],
///   "button blue ".to_string(),
/// );
///
/// let blue = false;
/// let disabled = true;
///
/// assert_eq!(
///   classnames![."button".blue.disabled],
///   "button disabled ".to_string(),
/// );
///
/// let is_blue = Some("blue");
/// let disabled = true;
///
/// assert_eq!(
///   classnames![."button".{is_blue}.disabled],
///   "button blue disabled ",
/// );
/// ```
#[macro_export]
macro_rules! classnames {
  [@single $result:ident <<] => {};

  // Handle string literals
  [@single $result:ident << .$str:literal $( $tt:tt )*] => {
    $crate::props::Classnames::append_to(&$str, &mut $result);
    classnames![@single $result << $( $tt ) *];
  };

  // Handle boolean variables
  [@single $result:ident << .$bool:ident $( $tt:tt )*] => {
    $crate::props::Classnames::append_to(
      &$bool.then(|| stringify!($bool)),
      &mut $result
    );
    classnames![@single $result << $( $tt ) *];
  };

  // Handle block expressions
  [@single $result:ident << .$block:block $( $tt:tt )*] => {
    $crate::props::Classnames::append_to(&$block, &mut $result);
    classnames![@single $result << $( $tt ) *];
  };

  [$( $tt:tt )*] => {
    {
      let mut result = String::new();
      classnames![@single result << $( $tt )*];
      result
    }
  };
}
