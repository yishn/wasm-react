use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Deps<T: PartialEq> {
  All,
  None,
  Some(Rc<T>),
}

impl<T: PartialEq> Clone for Deps<T> {
  fn clone(&self) -> Self {
    match self {
      Self::All => Self::All,
      Self::None => Self::None,
      Self::Some(deps) => Self::Some(deps.clone()),
    }
  }
}

#[macro_export]
macro_rules! deps {
  (*) => {
    $crate::hooks::Deps::All;
  };
  () => {
    $crate::hooks::Deps::None
  };
  ($expr:expr) => {
    $crate::hooks::Deps::Some(std::rc::Rc::new($expr))
  };
  ($( $expr:expr ),+ $(,)?) => {
    $crate::hooks::Deps::Some(std::rc::Rc::new(($( $expr ),+)))
  };
}
