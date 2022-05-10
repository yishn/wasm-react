use std::fmt::Debug;

/// This struct specifies dependencies for certain hooks.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// # fn log(s: &str) {}
/// # struct State { counter: () }
/// # struct F { id: () };
/// # impl F {
/// #   fn f(&self, state: State) {
/// #
/// use_effect(|| {
///   log("This effect will be called every time `self.id` or `state.counter` changes.");
///
///   || ()
/// }, Deps::some((self.id, state.counter)));
/// #
/// #   }
/// # }
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct Deps<T>(Option<T>);

impl<T: Debug> Debug for Deps<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut result = f.debug_tuple("Deps");

    match self.0.as_ref() {
      Some(deps) => result.field(&deps),
      None => result.field(&"All"),
    }
    .finish()
  }
}

impl Deps<()> {
  /// The hook will be activated whenever the component renders.
  pub fn all() -> Self {
    Self(None)
  }

  /// The hook will be activated only on the first render.
  pub fn none() -> Self {
    Self(Some(()))
  }
}

impl<T> Deps<T> {
  /// The hook will be activated every time when the component renders if the
  /// inner value `T` has changed from last render.
  pub fn some(deps: T) -> Self {
    Self(Some(deps))
  }

  pub(crate) fn is_all(&self) -> bool {
    self.0.is_none()
  }
}
