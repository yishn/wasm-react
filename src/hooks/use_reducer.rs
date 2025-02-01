use super::{use_ref, use_state, RefContainer, State, StateMut};
use crate::Callback;

pub struct Reducer<T, A>(StateMut<T>, RefContainer<Callback<(T, A), T>>);

impl<T, A> Reducer<T, A>
where
  T: 'static,
  A: 'static,
{
  pub fn dispatch(self, action: A) {
    self
      .0
      .update(|value| self.1.current().call((value, action)));
  }
}

impl<T, A> Clone for Reducer<T, A> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1.clone())
  }
}

impl<T, A> Copy for Reducer<T, A> {}

pub fn use_reducer<T, A>(
  mut reducer: impl FnMut(T, A) -> T + 'static,
  init: impl FnOnce() -> T,
) -> (State<T>, Reducer<T, A>)
where
  T: 'static,
  A: 'static,
{
  let (state, state_mut) = use_state(init);
  let reducer = Reducer(
    state_mut,
    use_ref(move || {
      Callback::new(move |(state, action)| reducer(state, action))
    }),
  );

  (state, reducer)
}
