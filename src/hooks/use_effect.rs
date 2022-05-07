use super::{use_ref, Deps};

pub fn use_effect<G, D>(mut f: impl FnMut() -> G, deps: Deps<D>)
where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  let mut effect = use_ref(None::<(G, Deps<D>)>);

  match effect.current_mut().take() {
    Some((destructor, old_deps)) => {
      if deps == Deps::All || old_deps != deps {
        destructor();
        effect.set_current(Some((f(), deps)));
      } else {
        effect.set_current(Some((destructor, old_deps)));
      }
    }
    None => {
      effect.set_current(Some((f(), deps)));
    }
  }
}
