use js_sys::Function;
use std::{fmt::Debug, ops::Deref, rc::Rc};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  prelude::Closure,
  JsValue,
};

/// Represents a callback with one and only one input argument and some return
/// value that can be passed to JS.
///
/// When converted into [`JsValue`], memory management will be transferred to
/// the JS garbage collection. This is facilitated through
/// [`FinalizationRegistry`][FinalizationRegistry].
///
/// **Remember** to set `WASM_BINDGEN_WEAKREF=1` before building with
/// `wasm-bindgen`, otherwise you will get memory leaks.
///
/// [FinalizationRegistry]: https://developer.mozilla.org/en-US/docs/Web/JS/Reference/Global_Objects/FinalizationRegistry
#[derive(Clone)]
pub struct Callback<T, U = ()>(Rc<dyn Fn(T) -> U>);

impl<T, U> Callback<T, U> {
  /// Constructs a new [`Callback`] from a Rust closure.
  pub fn new<F: Fn(T) -> U + 'static>(f: F) -> Self {
    Self(Rc::new(f))
  }

  /// Returns a new [`Callback`] which does nothing.
  pub fn noop() -> Callback<T> {
    Callback::new(|_: T| ())
  }
}

impl<T, U> Debug for Callback<T, U> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Callback(|_| { ... })")
  }
}

impl<T, U> Deref for Callback<T, U> {
  type Target = dyn Fn(T) -> U;

  fn deref(&self) -> &Self::Target {
    &*self.0
  }
}

impl<T, U> From<Callback<T, U>> for JsValue
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
{
  fn from(value: Callback<T, U>) -> Self {
    Closure::wrap(Box::new(move |arg| value.0(arg)) as Box<dyn Fn(T) -> U>)
      .into_js_value()
  }
}

/// A trait for callable structs with one and only one input argument and some
/// return value.
pub trait Callable<T, U> {
  /// Calls the struct with the given input argument.
  fn call(&self, arg: T) -> U;
}

impl<T, U, F: Fn(T) -> U> Callable<T, U> for F {
  fn call(&self, arg: T) -> U {
    self(arg)
  }
}

impl<T, U> Callable<T, U> for Callback<T, U> {
  fn call(&self, arg: T) -> U {
    self.0(arg)
  }
}

impl Callable<&JsValue, Result<JsValue, JsValue>> for Function {
  fn call(&self, arg: &JsValue) -> Result<JsValue, JsValue> {
    self.call1(&JsValue::undefined(), arg)
  }
}

impl<T, U: Default, F: Callable<T, U>> Callable<T, U> for Option<F> {
  fn call(&self, arg: T) -> U {
    self.as_ref().map(|f| f.call(arg)).unwrap_or_default()
  }
}
