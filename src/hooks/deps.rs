/// This specifies dependencies for certain hooks.
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
///   log("This effect will be called whenever this component renders.");
///
///   || ()
/// }, Deps::All::<()>);
///
/// use_effect(|| {
///   log("This effect will only be called once.");
///
///   || ()
/// }, Deps::None::<()>);
///
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
  All,
  None,
  Some(T),
}
