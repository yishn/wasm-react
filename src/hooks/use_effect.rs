use super::{use_ref_with_unmount_handler, Deps};

// FIXME: Currently does not call destructor when component unmounts.
///
pub fn use_effect<G, D>(effect: impl FnOnce() -> G, deps: Deps<D>)
where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  let mut ref_container =
    use_ref_with_unmount_handler(None::<(G, Deps<D>)>, |ref_container| {
      // Call the destructor when the component unmounts
      if let Some(destructor) =
        ref_container.current_mut().take().map(|(g, _)| g)
      {
        destructor();
      }
    });

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
