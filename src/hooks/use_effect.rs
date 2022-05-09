use super::{use_ref, Deps};
use crate::react_bindings;
use wasm_bindgen::prelude::Closure;

// FIXME: Currently does not call destructor when component unmounts.
pub fn use_effect<G, D>(effect: impl FnOnce() -> G, deps: Deps<D>)
where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  let mut ref_container = use_ref(None::<(G, Deps<D>)>);

  // This callback will always be called exactly one time.
  // This ensures that we call the destructor when the component unmounts.
  react_bindings::use_unmount_handler(&Closure::once_into_js({
    let mut ref_container = ref_container.clone();

    move |unmounted: bool| {
      if unmounted {
        if let Some(destructor) =
          ref_container.current_mut().take().map(|(g, _)| g)
        {
          destructor();
        }
      }
    }
  }));

  match ref_container.current_mut().take() {
    Some((destructor, old_deps)) => {
      if deps == Deps::All || old_deps != deps {
        // When dependencies change, call destructor and run effect
        destructor();
        ref_container.set_current(Some((effect(), deps)));
      } else {
        ref_container.set_current(Some((destructor, old_deps)));
      }
    }
    None => {
      ref_container.set_current(Some((effect(), deps)));
    }
  }
}
