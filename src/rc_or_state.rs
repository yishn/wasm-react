use crate::hooks::State;
use std::{cell::Ref, fmt::Debug, ops::Deref, rc::Rc};

/// Allows read-only access to the underlying value of [`RcOrState`].
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

/// This struct can either contain an [`Rc`] or a [`State`] struct and allows
/// read-only access to the underlying data.
///
/// This is useful as a prop type that may be coming from a mutable state.
/// Imagine you define a component with the following struct:
///
/// ```
/// # use std::rc::Rc;
/// struct TaskList {
///   tasks: Vec<Rc<str>>
/// }
/// ```
///
/// You want to include `TaskList` in a container component where `tasks` is
/// managed by a state:
///
/// ```compile_fail
/// # use wasm_react::{*, hooks::*};
/// # use std::rc::Rc;
/// #
/// # struct TaskList {
/// #   tasks: Vec<Rc<str>>
/// # }
/// # impl Component for TaskList {
/// #   fn render(&self) -> VNode { VNode::default() }
/// # }
/// # struct A;
/// # impl Component for A {
/// fn render(&self) -> VNode {
///   let tasks: State<Vec<Rc<str>>> = use_state(|| vec![]);
///
///   h!(div).build(c![
///     TaskList {
///       tasks: tasks.value(),
///     },
///   ])
/// }
/// # }
/// ```
///
/// But this doesn't compile, since `tasks.value()` returns a reference while
/// component structs can only contain `'static` values. You can clone the `Vec`,
/// but this introduces unnecessary overhead. In such a situation you might think
/// you can simply change the type of `TaskList` to a [`State`]:
///
/// ```
/// # use std::rc::Rc;
/// # use wasm_react::{*, hooks::*};
/// struct TaskList {
///   tasks: State<Vec<Rc<str>>>
/// }
///
/// /* ... */
///
/// # impl Component for TaskList {
/// #   fn render(&self) -> VNode { VNode::default() }
/// # }
/// # struct A;
/// # impl Component for A {
/// fn render(&self) -> VNode {
///   let tasks: State<Vec<Rc<str>>> = use_state(|| vec![]);
///
///   h!(div).build(c![
///     TaskList {
///       tasks,
///     }
///     .build(),
///   ])
/// }
/// # }
///  ```
///
/// This works as long as the prop `tasks` is guaranteed to come from a state.
/// But this assumption may not hold. This is where `RcOrState` comes in:
///
/// ```
/// # use std::rc::Rc;
/// # use wasm_react::{*, hooks::*};
/// struct TaskList {
///   tasks: RcOrState<Vec<Rc<str>>>
/// }
/// ```
///
/// By defining `tasks` to be `RcOrState`, you can either pass down a state or
/// pass down an [`Rc`] of your value.
#[derive(Debug)]
pub enum RcOrState<T: 'static> {
  /// Contains an [`Rc`].
  Rc(Rc<T>),
  /// Contains a [`State`].
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

impl<T> From<T> for RcOrState<T>
where
  T: Into<Rc<T>>,
{
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
