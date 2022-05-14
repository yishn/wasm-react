use crate::hooks::State;
use std::{cell::Ref, fmt::Debug, ops::Deref, rc::Rc};

pub enum ReadOrStateValueRef<'a, T: 'static> {
  ReadOnly(&'a Rc<T>),
  State(Ref<'a, T>),
}

impl<'a, T> Deref for ReadOrStateValueRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self {
      ReadOrStateValueRef::ReadOnly(x) => x.deref(),
      ReadOrStateValueRef::State(x) => x.deref(),
    }
  }
}

#[derive(Debug)]
pub enum ReadOrState<T: 'static> {
  ReadOnly(Rc<T>),
  State(State<T>),
}

impl<T> ReadOrState<T> {
  pub fn value(&self) -> ReadOrStateValueRef<'_, T> {
    match &self {
      ReadOrState::ReadOnly(x) => ReadOrStateValueRef::ReadOnly(x),
      ReadOrState::State(x) => ReadOrStateValueRef::State(x.value()),
    }
  }
}

impl<T> Clone for ReadOrState<T> {
  fn clone(&self) -> Self {
    match self {
      Self::ReadOnly(x) => Self::ReadOnly(x.clone()),
      Self::State(x) => Self::State(x.clone()),
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
