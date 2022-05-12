use super::{use_ref, Deps};
use crate::{
  callback::{Callback, Void},
  react_bindings,
};
use wasm_bindgen::{prelude::Closure, JsValue, UnwrapThrowExt};

fn use_effect_inner<G, D>(
  effect: impl FnOnce() -> G + 'static,
  deps: Deps<D>,
  f: impl Fn(&JsValue, u8),
) where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  let effect = Callback::once(move |_: Void| {
    let destructor = effect();

    // The effect destructor will definitely be called exactly once by React
    Closure::once_into_js(move |_: Void| destructor())
  });
  let mut ref_container =
    use_ref(None::<(Callback<Void, JsValue>, Deps<D>, u8)>);

  match ref_container.current_mut().as_mut() {
    Some((old_effect, old_deps, counter)) => {
      if deps.is_all() || old_deps != &deps {
        *old_effect = effect;
        *old_deps = deps;
        *counter = counter.wrapping_add(1);
      }
    }
    None => {
      ref_container.set_current(Some((effect, deps, 0)));
    }
  };

  let (effect, _, counter) = ref_container.current().as_ref().unwrap_throw();

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
/// let state = use_state(|| None);
///
/// use_effect({
///   let mut state = state.clone();
///   let url = self.url;
///
///   move || {
///     state.set(|_| Some(fetch(url)));
///     || ()
///   }
/// }, Deps::some(self.url));
/// #
/// #   }
/// # }
/// ```
pub fn use_effect<G, D>(effect: impl FnOnce() -> G + 'static, deps: Deps<D>)
where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  use_effect_inner(effect, deps, react_bindings::use_rust_effect);
}

/// Like [`use_effect()`], but it fires synchronously after all DOM mutations.
///
/// See [React documentation](https://reactjs.org/docs/hooks-reference.html#uselayouteffect).
pub fn use_layout_effect<G, D>(
  effect: impl FnOnce() -> G + 'static,
  deps: Deps<D>,
) where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  use_effect_inner(effect, deps, react_bindings::use_rust_layout_effect);
}
