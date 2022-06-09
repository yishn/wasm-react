use std::{
  cell::{Ref, RefCell},
  ops::Deref,
  rc::Rc,
};

/// Allows read-only access to the underlying value of [`ValueContainer`].
#[non_exhaustive]
pub enum ValueContainerRef<'a, T> {
  /// Contains an immutable borrow.
  Simple(&'a T),
  /// Contains a [`Ref`] reference.
  Ref(Ref<'a, T>),
}

impl<'a, T> ValueContainerRef<'a, T> {
  /// Clones the reference.
  pub fn clone(orig: &Self) -> Self {
    match orig {
      ValueContainerRef::Simple(x) => ValueContainerRef::Simple(x),
      ValueContainerRef::Ref(x) => ValueContainerRef::Ref(Ref::clone(x)),
    }
  }
}

impl<'a, T> Deref for ValueContainerRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self {
      ValueContainerRef::Simple(x) => x,
      ValueContainerRef::Ref(x) => x.deref(),
    }
  }
}

/// An abstraction over structs that contain a value which can be accessed
/// through an immutable borrow or [`Ref`].
///
/// All hook containers implement this trait and also [`Rc<T>`] and
/// [`Rc<RefCell<T>>`].
pub trait ValueContainer<T>: 'static {
  /// Returns a read-only reference to the underlying value.
  fn value(&self) -> ValueContainerRef<'_, T>;
}

impl<T: 'static> ValueContainer<T> for T {
  fn value(&self) -> ValueContainerRef<'_, T> {
    ValueContainerRef::Simple(&self)
  }
}

impl<T: 'static> ValueContainer<T> for &'static T {
  fn value(&self) -> ValueContainerRef<'_, T> {
    ValueContainerRef::Simple(self)
  }
}

impl<T: 'static> ValueContainer<T> for Rc<T> {
  fn value(&self) -> ValueContainerRef<'_, T> {
    ValueContainerRef::Simple(&**self)
  }
}

impl<T: 'static> ValueContainer<T> for Rc<RefCell<T>> {
  fn value(&self) -> ValueContainerRef<'_, T> {
    ValueContainerRef::Ref(self.borrow())
  }
}

/// A struct wrapper for the [`ValueContainer`] trait.
pub struct ValContainer<T>(Box<dyn ValueContainer<T>>);

impl<T> ValContainer<T> {
  /// Creates a new [`ValContainer`] struct from a [`ValueContainer`].
  pub fn new(value: impl ValueContainer<T>) -> Self {
    Self(Box::new(value))
  }
}

impl<T> Deref for ValContainer<T> {
  type Target = dyn ValueContainer<T>;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}
