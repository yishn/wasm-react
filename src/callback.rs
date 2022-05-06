use js_sys::Function;
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  describe::WasmDescribe,
  prelude::Closure,
  JsCast, JsValue, UnwrapThrowExt,
};

/// A helper struct to simulate a [`Callback`] with no input arguments.
///
/// ```
/// let callback: Callback<Void> = Callback::new(|_: Void| ());
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Void;

impl WasmDescribe for Void {
  fn describe() {
    JsValue::describe()
  }
}

impl FromWasmAbi for Void {
  type Abi = <JsValue as FromWasmAbi>::Abi;

  unsafe fn from_abi(js: Self::Abi) -> Self {
    JsValue::from_abi(js);
    Void
  }
}

impl From<Void> for JsValue {
  fn from(_: Void) -> Self {
    JsValue::undefined()
  }
}

/// This is a typed wrapper around a JS [`Function`] that represents a callback
/// with one and only one input argument and some return value.
///
/// When constructed from a Rust closure, memory management of the closure will
/// be transferred to the JS garbage collection. This is facilitated through
/// [`FinalizationRegistry`][FinalizationRegistry].
///
/// **Remember** to set `WASM_BINDGEN_WEAKREF=1` before building with
/// `wasm-bindgen`, otherwise you will get memory leaks.
///
/// [FinalizationRegistry]: https://developer.mozilla.org/en-US/docs/Web/JS/Reference/Global_Objects/FinalizationRegistry
#[derive(Default, Clone)]
pub struct Callback<T, U = ()>(Function, PhantomData<(T, U)>);

impl<T, U> Callback<T, U> {
  /// Constructs a new [`Callback`] from a Rust closure.
  pub fn new(f: impl Fn(T) -> U + 'static) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    Self(
      Closure::wrap(Box::new(move |arg| f(arg)) as Box<dyn Fn(T) -> U>)
        .into_js_value()
        .dyn_into::<Function>()
        .unwrap_throw(),
      PhantomData,
    )
  }

  /// Returns a [`Callback`] from a JS [`Function`].
  pub fn from_function(f: Function) -> Self {
    Self(f, PhantomData)
  }
}

impl<T, U> Debug for Callback<T, U> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Callback(|_| { ... })")
  }
}

impl<T, U> AsRef<Function> for Callback<T, U> {
  fn as_ref(&self) -> &Function {
    &self.0
  }
}

impl<T, U> AsRef<JsValue> for Callback<T, U> {
  fn as_ref(&self) -> &JsValue {
    self.0.as_ref()
  }
}

impl<T, U> From<Callback<T, U>> for JsValue {
  fn from(value: Callback<T, U>) -> Self {
    value.0.into()
  }
}

impl<T, U> From<Callback<T, U>> for Function {
  fn from(value: Callback<T, U>) -> Self {
    value.0
  }
}

impl<T, U, F> From<F> for Callback<T, U>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
  F: Fn(T) -> U + 'static,
{
  fn from(value: F) -> Self {
    Callback::new(value)
  }
}

impl<T, U> PartialEq for Callback<T, U> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T, U> Eq for Callback<T, U> {}

/// A trait for callable structs with one and only one input argument and some
/// return value.
pub trait Callable<T, U = ()> {
  /// Calls the struct with the given input argument.
  fn call(&self, arg: T) -> U;
}

impl<T, U, F: Fn(T) -> U> Callable<T, U> for F {
  fn call(&self, arg: T) -> U {
    self(arg)
  }
}

impl<T> Callable<T, ()> for Callback<T, ()>
where
  T: Into<JsValue>,
{
  fn call(&self, arg: T) {
    self.0.call1(&JsValue::undefined(), &arg.into()).unwrap_throw();
  }
}

impl<T> Callable<T, JsValue> for Callback<T, JsValue>
where
  T: Into<JsValue>,
{
  fn call(&self, arg: T) -> JsValue {
    self.0.call1(&JsValue::undefined(), &arg.into()).unwrap_throw()
  }
}

impl Callable<&JsValue, JsValue> for Function {
  fn call(&self, arg: &JsValue) -> JsValue {
    self.call1(&JsValue::undefined(), arg).unwrap_throw()
  }
}

impl Callable<JsValue, JsValue> for Function {
  fn call(&self, arg: JsValue) -> JsValue {
    self.call1(&JsValue::undefined(), &arg).unwrap_throw()
  }
}

impl<T, U: Default, F: Callable<T, U>> Callable<T, U> for Option<F> {
  fn call(&self, arg: T) -> U {
    self.as_ref().map(|f| f.call(arg)).unwrap_or_default()
  }
}
