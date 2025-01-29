use super::{use_ref, Deps};
use crate::react_bindings;
use wasm_bindgen::{prelude::Closure, JsValue, UnwrapThrowExt};

/// Denotes types that can be used as destructors for effects.
pub trait Destructor {
  /// Cleans up the effect.
  fn clean_up(self);
}

impl Destructor for () {
  fn clean_up(self) {
    // Do nothing
  }
}

impl<F> Destructor for F
where
  F: FnOnce() + 'static,
{
  fn clean_up(self) {
    self()
  }
}

fn use_effect_inner<F, D>(
  mut f: impl FnMut() -> F + 'static,
  deps: Deps<D>,
  use_effect: fn(&JsValue, u8),
) where
  F: Destructor + 'static,
  D: PartialEq + 'static,
{
  let f_js = Closure::new(move || {
    let destructor = f();

    // The effect destructor will be called exactly once by React
    Closure::once_into_js(move || destructor.clean_up())
  })
  .into_js_value();

  let ref_container = use_ref(|| None::<(Deps<D>, u8)>);

  let new_value = match ref_container.current_mut().take() {
    Some((old_deps, counter)) => {
      if deps.is_all() || old_deps != deps {
        Some((deps, counter.wrapping_add(1)))
      } else {
        Some((old_deps, counter))
      }
    }
    None => Some((deps, 0)),
  };
  ref_container.set_current(new_value);

  use_effect(&f_js, ref_container.current().as_ref().unwrap_throw().1);
}

pub fn use_effect<F, D>(f: impl FnMut() -> F + 'static, deps: Deps<D>)
where
  F: Destructor + 'static,
  D: PartialEq + 'static,
{
  use_effect_inner(f, deps, react_bindings::use_effect);
}

pub fn use_layout_effect<F, D>(f: impl FnMut() -> F + 'static, deps: Deps<D>)
where
  F: Destructor + 'static,
  D: PartialEq + 'static,
{
  use_effect_inner(f, deps, react_bindings::use_layout_effect);
}

pub fn use_insertion_effect<F, D>(f: impl FnMut() -> F + 'static, deps: Deps<D>)
where
  F: Destructor + 'static,
  D: PartialEq + 'static,
{
  use_effect_inner(f, deps, react_bindings::use_insertion_effect);
}
