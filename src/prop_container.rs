use crate::hooks::{DeferredValue, Memo, RefContainer, State};
use std::{
  cell::{Ref, RefCell},
  ops::Deref,
  rc::Rc,
};

/// Allows read-only access to the underlying value of [`PropContainer`].
#[non_exhaustive]
#[derive(Debug)]
pub enum PropContainerRef<'a, T> {
  #[allow(missing_docs)]
  Simple(&'a T),
  #[allow(missing_docs)]
  Ref(Ref<'a, T>),
}

impl<T> PropContainerRef<'_, T> {
  /// Clones the reference.
  pub fn clone(orig: &Self) -> Self {
    match orig {
      PropContainerRef::Simple(x) => PropContainerRef::Simple(x),
      PropContainerRef::Ref(x) => PropContainerRef::Ref(Ref::clone(x)),
    }
  }
}

impl<T> Deref for PropContainerRef<'_, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self {
      PropContainerRef::Simple(x) => x,
      PropContainerRef::Ref(x) => x.deref(),
    }
  }
}

macro_rules! define_value_container {
  {
    $(
      $Variant:ident($id:ident: $Ty:ty) => $RefVariant:ident($expr:expr) $(,)?
    )*
  } => {
    /// A helpful abstraction over non-`Copy` types that can be used as a prop
    /// type for components.
    ///
    /// Can contain all hook containers, [`Rc<T>`], and [`Rc<RefCell<T>>`].
    #[non_exhaustive]
    #[derive(Debug)]
    pub enum PropContainer<T> {
      $(
        #[allow(missing_docs)]
        $Variant($Ty),
      )*
    }

    impl<T: 'static> PropContainer<T> {
      /// Returns a read-only reference to the underlying value.
      pub fn value(&self) -> PropContainerRef<'_, T> {
        match self {
          $( Self::$Variant($id) => PropContainerRef::$RefVariant($expr), )*
        }
      }
    }

    impl<T> Clone for PropContainer<T> {
      fn clone(&self) -> Self {
        match self {
          $( Self::$Variant(x) => Self::$Variant(x.clone()), )*
        }
      }
    }

    $(
      impl<T> From<$Ty> for PropContainer<T> {
        fn from(value: $Ty) -> Self {
          Self::$Variant(value)
        }
      }
    )*
  };
}

define_value_container! {
  Rc(x: Rc<T>) => Simple(x.deref()),
  RcRefCell(x: Rc<RefCell<T>>) => Ref(x.borrow()),
  RefContainer(x: RefContainer<T>) => Ref(x.current()),
  State(x: State<T>) => Ref(x.value()),
  Memo(x: Memo<T>) => Ref(x.value()),
  DeferredValue(x: DeferredValue<T>) => Ref(x.value()),
}

impl<T: PartialEq + 'static> PartialEq for PropContainer<T> {
  fn eq(&self, other: &Self) -> bool {
    T::eq(&self.value(), &other.value())
  }
}

impl<T: PartialEq + 'static> PartialEq<T> for PropContainer<T> {
  fn eq(&self, other: &T) -> bool {
    T::eq(&self.value(), other)
  }
}

impl<T> From<T> for PropContainer<T> {
  fn from(value: T) -> Self {
    Self::from(Rc::new(value))
  }
}
