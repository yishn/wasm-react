use crate::hooks::State;
use std::{fmt::Debug, ops::Deref, rc::Rc};

/// Represents
pub enum ReadOrState<T: 'static> {
  ReadOnly(Rc<T>),
  State(State<T>),
}

impl<T> Clone for ReadOrState<T> {
  fn clone(&self) -> Self {
    match self {
      Self::ReadOnly(x) => Self::ReadOnly(x.clone()),
      Self::State(x) => Self::State(x.clone()),
    }
  }
}

impl<T: Debug> Debug for ReadOrState<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

impl<T> Deref for ReadOrState<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self {
      ReadOrState::ReadOnly(x) => x.deref(),
      ReadOrState::State(x) => x.deref(),
    }
  }
}

impl<T> From<T> for ReadOrState<T> {
  fn from(value: T) -> Self {
    ReadOrState::ReadOnly(Rc::new(value))
  }
}

impl<T> From<State<T>> for ReadOrState<T> {
  fn from(value: State<T>) -> Self {
    ReadOrState::State(value)
  }
}

impl<T> From<Rc<T>> for ReadOrState<T> {
  fn from(value: Rc<T>) -> Self {
    ReadOrState::ReadOnly(value)
  }
}
