/// A marker trait for data that persist through the entire lifetime of a
/// component, usually through a hook.
///
/// This means, even if a struct with this trait itself is dropped, the
/// underlying data is still available in the memory until the component is
/// unmounted by React.
///
/// Cloning a struct with this trait will not duplicate the underlying data, but
/// cloning can be used to make the underlying data available for a closure, by
/// moving a clone inside it.
pub trait Persisted: Clone {
  fn ptr(&self) -> PersistedOrigin;
}

#[doc(hidden)]
#[non_exhaustive]
pub struct PersistedOrigin;
