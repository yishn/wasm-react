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
/// use_effect(|| {
///   log("This effect will be called every time `self.id` or `state.counter` changes.");
///
///   || ()
/// }, Deps::Some((self.id, state.counter)));
/// #   }
/// # }
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Deps<T: PartialEq> {
  /// The hook will be activated whenever the component renders.
  All,
  /// The hook will be activated only on the first render.
  None,
  /// The hook will be activated every time when the component renders if the
  /// inner value `T` has changed from last render.
  Some(T),
}
