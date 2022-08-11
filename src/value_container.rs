use crate::hooks::{DeferredValue, Memo, RefContainer, State};
use std::{
  cell::{Ref, RefCell},
  ops::Deref,
  rc::Rc,
};

/// Allows read-only access to the underlying value of [`ValueContainer`].
#[non_exhaustive]
#[derive(Debug)]
pub enum ValueContainerRef<'a, T> {
  #[doc(hidden)]
  #[non_exhaustive]
  Simple(&'a T),
  #[doc(hidden)]
  #[non_exhaustive]
  Ref(Ref<'a, T>),
}

impl<T> ValueContainerRef<'_, T> {
  /// Clones the reference.
  pub fn clone(orig: &Self) -> Self {
    match orig {
      ValueContainerRef::Simple(x) => ValueContainerRef::Simple(x),
      ValueContainerRef::Ref(x) => ValueContainerRef::Ref(Ref::clone(x)),
    }
  }
}

impl<T> Deref for ValueContainerRef<'_, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self {
      ValueContainerRef::Simple(x) => x,
      ValueContainerRef::Ref(x) => x.deref(),
    }
  }
}

macro_rules! define_value_container {
  {
    $(
      $Variant:ident($id:ident: $Ty:ty) => $RefVariant:ident($expr:expr) $(,)?
    )*
  } => {
    /// An abstraction over structs that contain a value which can be accessed
    /// through an immutable borrow or [`Ref`].
    ///
    /// Can contain all hook containers and [`Rc<T>`], [`Rc<RefCell<T>>`].
    #[non_exhaustive]
    #[derive(Debug)]
    pub enum ValueContainer<T> {
      $(
        #[doc(hidden)]
        #[non_exhaustive]
        $Variant($Ty),
      )*
    }

    impl<T: 'static> ValueContainer<T> {
      /// Returns a read-only reference to the underlying value.
      pub fn value(&self) -> ValueContainerRef<'_, T> {
        match self {
          $( Self::$Variant($id) => ValueContainerRef::$RefVariant($expr), )*
        }
      }
    }

    impl<T> Clone for ValueContainer<T> {
      fn clone(&self) -> Self {
        match self {
          $( Self::$Variant(x) => Self::$Variant(x.clone()), )*
        }
      }
    }

    $(
      impl<T> From<$Ty> for ValueContainer<T> {
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

impl<T: PartialEq + 'static> PartialEq for ValueContainer<T> {
  fn eq(&self, other: &Self) -> bool {
    T::eq(&self.value(), &other.value())
  }
}

impl<T> From<T> for ValueContainer<T> {
  fn from(value: T) -> Self {
    Self::from(Rc::new(value))
  }
}
