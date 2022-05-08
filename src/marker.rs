/// A marker trait for data that can persist through the entire lifetime of a
/// component, usually through a hook.
///
/// This means, even if a struct with this trait itself is dropped, the
/// underlying data is still available in the memory until the component is
/// unmounted by React or replaced by some other data.
///
/// Cloning a struct with this trait will not duplicate the underlying data, but
/// only a pointer, so cloning is a fast way to make the underlying data
/// available for a closure, by moving a clone of the pointer inside it.
pub trait Persisted: Clone {
  fn ptr(&self) -> PersistedOrigin;
}

#[doc(hidden)]
#[non_exhaustive]
pub struct PersistedOrigin;
