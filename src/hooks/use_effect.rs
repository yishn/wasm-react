use super::{use_ref, Deps};

pub fn use_effect<G, D>(mut f: impl FnMut() -> G, deps: Deps<D>)
where
  G: FnOnce() + 'static,
  D: PartialEq + 'static,
{
  let mut effect = use_ref(None::<(G, Deps<D>)>);

  match effect.current.take() {
    Some((destructor, old_deps)) => {
      if deps == Deps::All || old_deps != deps {
        destructor();
        effect.current = Some((f(), deps));
      } else {
        effect.current = Some((destructor, old_deps));
      }
    }
    None => {
      effect.current = Some((f(), deps));
    }
  }
}
