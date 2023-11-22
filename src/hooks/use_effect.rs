use super::{use_ref, Deps};
use crate::react_bindings;
use wasm_bindgen::{prelude::Closure, JsValue, UnwrapThrowExt};

/// Denotes types that can be used as destructors for effects.
pub trait IntoDestructor {
  #[doc(hidden)]
  type Destructor: FnOnce() + 'static;

  #[doc(hidden)]
  fn into_destructor(self) -> Self::Destructor;
}

impl IntoDestructor for () {
  type Destructor = fn();

  fn into_destructor(self) -> Self::Destructor {
    || ()
  }
}

impl<F> IntoDestructor for F
where
  F: FnOnce() + 'static,
{
  type Destructor = fn();

  fn into_destructor(self) -> Self::Destructor {
    || ()
  }
}

fn use_effect_inner<G, D>(
  effect: impl FnOnce() -> G + 'static,
  deps: Deps<D>,
  f: impl FnOnce(&JsValue, u8),
) where
  G: IntoDestructor,
  D: PartialEq + 'static,
{
  let create_effect_closure = move || {
    Closure::once(move || {
      let destructor = effect();

      // The effect destructor will definitely be called exactly once by React
      Closure::once_into_js(destructor.into_destructor())
    })
  };

  let mut ref_container =
    use_ref(None::<(Closure<dyn FnMut() -> JsValue>, Deps<D>, u8)>);

  let new_value = match ref_container.current_mut().take() {
    Some((old_effect, old_deps, counter)) => {
      if deps.is_all() || old_deps != deps {
        Some((create_effect_closure(), deps, counter.wrapping_add(1)))
      } else {
        // Dependencies didn't change
        Some((old_effect, old_deps, counter))
      }
    }
    None => Some((create_effect_closure(), deps, 0)),
  };

  ref_container.set_current(new_value);

  let value = ref_container.current();
  let (effect, _, counter) =
    value.as_ref().expect_throw("no effect data available");

  f(effect.as_ref(), *counter);
}

/// Runs a function which contains imperative code that may cause side-effects.
///
/// The given function will run after render is committed to the screen when
/// the given dependencies have changed from last render. The function can
/// return a clean-up function.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// #
/// # fn fetch(url: &str) -> String { String::new() }
/// # struct C { url: &'static str }
/// # impl C {
/// #   fn f(&self) {
/// let url = self.url;
/// let state = use_state(|| None);
///
/// use_effect({
///   clones!(mut state);
///   move || {
///     state.set(|_| Some(fetch(url)));
///   }
/// }, Deps::some(self.url));
/// #
/// #   }
/// # }
/// ```
pub fn use_effect<G, D>(effect: impl FnOnce() -> G + 'static, deps: Deps<D>)
where
  G: IntoDestructor,
  D: PartialEq + 'static,
{
  use_effect_inner(effect, deps, react_bindings::use_rust_effect);
}

/// Same as [`use_effect()`], but it fires synchronously after all DOM mutations.
///
/// See [React documentation](https://react.dev/reference/react/useLayoutEffect).
pub fn use_layout_effect<G, D>(
  effect: impl FnOnce() -> G + 'static,
  deps: Deps<D>,
) where
  G: IntoDestructor,
  D: PartialEq + 'static,
{
  use_effect_inner(effect, deps, react_bindings::use_rust_layout_effect);
}
