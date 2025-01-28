/// A trait for types to be used in [`classnames!`](crate::classnames!).
pub trait Classname {
  /// Appends the class to a string.
  fn append_to(&self, string: &mut String);
}

impl Classname for &str {
  fn append_to(&self, string: &mut String) {
    if !string.is_empty() {
      string.push(' ');
    }

    string.push_str(self);
  }
}

impl Classname for String {
  fn append_to(&self, string: &mut String) {
    (&self[..]).append_to(string);
  }
}

impl Classname for &String {
  fn append_to(&self, string: &mut String) {
    (&self[..]).append_to(string);
  }
}

impl<T: Classname> Classname for Option<T> {
  fn append_to(&self, string: &mut String) {
    if let Some(value) = self {
      value.append_to(string);
    }
  }
}

#[macro_export]
macro_rules! classnames {
  (@single $result:ident <<) => {};

  // Handle string literals
  (@single $result:ident << .$str:literal $( $tail:tt )*) => {
    $crate::Classname::append_to(&::wasm_bindgen::intern($str), &mut $result);
    $crate::classnames!(@single $result << $( $tail ) *);
  };

  // Handle boolean variables
  (@single $result:ident << .$bool:ident $( $tail:tt )*) => {
    $crate::Classname::append_to(
      &$bool.then(|| ::wasm_bindgen::intern(stringify!($bool))),
      &mut $result
    );
    $crate::classnames!(@single $result << $( $tail ) *);
  };

  // Handle block expressions
  (@single $result:ident << .$block:block $( $tail:tt )*) => {
    $crate::Classname::append_to(&$block, &mut $result);
    $crate::classnames!(@single $result << $( $tail ) *);
  };

  ($( $tt:tt )*) => {
    {
      let mut result = ::std::string::String::new();
      $crate::classnames!(@single result << $( $tt )*);
      result
    }
  };
}
