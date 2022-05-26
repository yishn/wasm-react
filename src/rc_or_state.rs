use crate::hooks::State;
use std::{cell::Ref, fmt::Debug, ops::Deref, rc::Rc};

/// Allows read-only access to the underlying value.
pub enum RcOrStateRef<'a, T: 'static> {
  /// Contains a reference to an [`Rc`].
  Rc(&'a Rc<T>),
  /// Contains a [`State`] reference.
  State(Ref<'a, T>),
}

impl<'a, T> Deref for RcOrStateRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self {
      RcOrStateRef::Rc(x) => x.deref(),
      RcOrStateRef::State(x) => x.deref(),
    }
  }
}

#[derive(Debug)]
pub enum RcOrState<T: 'static> {
  Rc(Rc<T>),
  State(State<T>),
}

impl<T> RcOrState<T> {
  /// Returns a read-only reference to the underlying value.
  pub fn value(&self) -> RcOrStateRef<'_, T> {
    match &self {
      RcOrState::Rc(x) => RcOrStateRef::Rc(x),
      RcOrState::State(x) => RcOrStateRef::State(x.value()),
    }
  }
}

impl<T> Clone for RcOrState<T> {
  fn clone(&self) -> Self {
    match self {
      Self::Rc(x) => Self::Rc(x.clone()),
      Self::State(x) => Self::State(x.clone()),
    }
  }
}

impl<T: Into<Rc<T>>> From<T> for RcOrState<T> {
  fn from(value: T) -> Self {
    RcOrState::Rc(value.into())
  }
}

impl<T> From<State<T>> for RcOrState<T> {
  fn from(value: State<T>) -> Self {
    RcOrState::State(value)
  }
}

impl<T> From<Rc<T>> for RcOrState<T> {
  fn from(value: Rc<T>) -> Self {
    RcOrState::Rc(value)
  }
}
