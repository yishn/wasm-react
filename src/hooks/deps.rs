use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub enum Deps<T: Eq> {
  All,
  None,
  Some(Rc<T>),
}

impl<T: Eq> Clone for Deps<T> {
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
    Deps::All;
  };
  () => {
    Deps::None
  };
  ($( $expr:expr ),+ $(,)?) => {
    Deps::Some(Rc::new(($( $expr ),+)))
  };
}
