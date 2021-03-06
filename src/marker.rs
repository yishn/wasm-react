use js_sys::JsString;
use wasm_bindgen::JsValue;

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
pub trait Persisted: Clone + 'static {
  #[doc(hidden)]
  fn ptr(&self) -> PersistedOrigin;
}

#[doc(hidden)]
#[non_exhaustive]
pub struct PersistedOrigin;

/// Implemented by types which can serve as a [React key][key].
///
/// [key]: https://reactjs.org/docs/lists-and-keys.html
pub trait KeyType: Into<JsValue> {}

macro_rules! impl_key_type {
  { $( $T:ty ),* $( , )? } => {
    $( impl KeyType for $T {} )*
  };
}

impl_key_type! {
  &str, String, JsString,
  f32, f64,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
}
