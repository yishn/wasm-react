use super::{use_ref, Deps};
use crate::{
  callback::{Callback, Void},
  react_bindings,
};
use wasm_bindgen::{prelude::Closure, JsValue, UnwrapThrowExt};

pub fn use_effect<G, D>(effect: impl FnOnce() -> G + 'static, deps: Deps<D>)
where
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

  react_bindings::use_rust_effect(effect.as_ref(), *counter);
}
