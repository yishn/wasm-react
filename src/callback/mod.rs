//! This module provides structs to pass Rust closures to JS.

use crate::{Persisted, PersistedOrigin};
use js_sys::Function;
use std::{
  cell::{Ref, RefCell},
  fmt::Debug,
  ops::Deref,
  rc::Rc,
};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  describe::WasmDescribe,
  prelude::Closure,
  JsValue, UnwrapThrowExt,
};

/// A helper struct to simulate a JS-interoperable [`Callback`] with no input
/// arguments.
///
/// ```
/// # use wasm_react::callback::*;
/// # fn f() {
/// let callback: Callback<Void> = Callback::new(|_: Void| ());
/// # }
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

/// This is a simplified, reference-counted wrapper around an [`FnMut`] Rust
/// closure that may be called from JS when `T` and `U` allow.
///
/// It only supports closures with exactly one input argument and some return
/// value. Memory management is handled by Rust. Whenever Rust drops all clones
/// of the [`Callback`], the closure will be dropped and the function cannot be
/// called from JS anymore.
///
/// Use [`Void`] to simulate a callback with no arguments.
pub struct Callback<T, U = ()> {
  closure: Rc<RefCell<dyn FnMut(T) -> U>>,
  js: Rc<RefCell<Option<Closure<dyn FnMut(T) -> U>>>>,
}

impl<T, U> Callback<T, U>
where
  T: 'static,
  U: 'static,
{
  /// Creates a new [`Callback`] from a Rust closure.
  pub fn new(f: impl FnMut(T) -> U + 'static) -> Self {
    Self {
      closure: Rc::new(RefCell::new(f)),
      js: Default::default(),
    }
  }

  /// Returns a Rust closure from the callback.
  pub fn to_closure(&self) -> impl FnMut(T) -> U + 'static {
    let closure = self.closure.clone();

    move |arg| {
      let mut f = closure.borrow_mut();
      f(arg)
    }
  }

  /// Returns a new [`Callback`] by prepending the given closure to the callback.
  pub fn premap<V>(
    &self,
    mut f: impl FnMut(V) -> T + 'static,
  ) -> Callback<V, U> {
    Callback {
      closure: Rc::new(RefCell::new({
        let closure = self.closure.clone();

        move |v| {
          let t = f(v);
          let mut g = closure.borrow_mut();
          g(t)
        }
      })),
      js: Default::default(),
    }
  }

  /// Returns a new [`Callback`] by appending the given closure to the callback.
  pub fn postmap<V>(
    &self,
    mut f: impl FnMut(U) -> V + 'static,
  ) -> Callback<T, V> {
    Callback {
      closure: Rc::new(RefCell::new({
        let closure = self.closure.clone();

        move |t| {
          let mut g = closure.borrow_mut();
          let u = g(t);
          f(u)
        }
      })),
      js: Default::default(),
    }
  }

  /// Returns a reference to `JsValue` of the callback.
  pub fn as_js(&self) -> Ref<'_, JsValue>
  where
    T: FromWasmAbi,
    U: IntoWasmAbi,
  {
    {
      let mut borrow = self.js.borrow_mut();

      if borrow.is_none() {
        *borrow = Some(Closure::wrap(Box::new({
          let closure = self.closure.clone();

          move |arg| {
            let mut f = closure.borrow_mut();
            f(arg)
          }
        })));
      }
    }

    Ref::map(self.js.borrow(), |x| {
      x.as_ref().expect_throw("no closure available").as_ref()
    })
  }
}

impl<T: 'static> Callback<T> {
  /// Returns a new [`Callback`] that does nothing.
  pub fn noop() -> Self {
    Callback::new(|_| ())
  }
}

impl<T, U> Default for Callback<T, U>
where
  T: 'static,
  U: Default + 'static,
{
  fn default() -> Self {
    Self::new(|_| U::default())
  }
}

impl<T, U> Debug for Callback<T, U> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Callback(|_| { … })")
  }
}

impl<T, U> PartialEq for Callback<T, U> {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.closure, &other.closure) && Rc::ptr_eq(&self.js, &other.js)
  }
}

impl<T, U> Eq for Callback<T, U> {}

impl<T, U> Clone for Callback<T, U> {
  fn clone(&self) -> Self {
    Self {
      closure: self.closure.clone(),
      js: self.js.clone(),
    }
  }
}

impl<T, U> Callable<T, U> for Callback<T, U> {
  fn call(&self, arg: T) -> U {
    let mut f = self.closure.borrow_mut();
    f(arg)
  }
}

/// This is a wrapper around a [`Callback`] which can persist through the
/// lifetime of a component.
///
/// Usually, this struct is created by using the
/// [`use_callback()`](crate::hooks::use_callback()) hook.
///
/// As with [`Callback`], this only supports closures with exactly one input
/// argument and some return value. The underlying Rust closure will be dropped
/// when all of the following conditions are met:
///
/// - All clones have been dropped.
/// - All clones to the underlying [`Callback`] have been dropped.
/// - The React component has unmounted.
///
/// It can be dropped earlier, e.g. when the underlying [`Callback`] has been
/// replaced by another and no more clones exist.
pub struct PersistedCallback<T, U = ()>(pub(crate) Callback<T, U>);

impl<T, U> Debug for PersistedCallback<T, U> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("PersistedCallback(|_| { … })")
  }
}

impl<T, U> Clone for PersistedCallback<T, U> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T, U> PartialEq for PersistedCallback<T, U> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T, U> Eq for PersistedCallback<T, U> {}

impl<T, U> Deref for PersistedCallback<T, U> {
  type Target = Callback<T, U>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T, U> From<PersistedCallback<T, U>> for Callback<T, U> {
  fn from(value: PersistedCallback<T, U>) -> Self {
    value.0
  }
}

impl<T, U> AsRef<Callback<T, U>> for PersistedCallback<T, U> {
  fn as_ref(&self) -> &Callback<T, U> {
    &self.0
  }
}

impl<T: 'static, U: 'static> Persisted for PersistedCallback<T, U> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

impl<T, U> Callable<T, U> for PersistedCallback<T, U> {
  fn call(&self, arg: T) -> U {
    self.deref().call(arg)
  }
}

/// A trait for callable structs with one and only one input argument and some
/// return value.
pub trait Callable<T, U = ()> {
  /// Calls the struct with the given input argument.
  fn call(&self, arg: T) -> U;
}

impl<T, U, F> Callable<T, U> for F
where
  F: Fn(T) -> U,
{
  fn call(&self, arg: T) -> U {
    self(arg)
  }
}

impl Callable<&JsValue, Result<JsValue, JsValue>> for Function {
  fn call(&self, arg: &JsValue) -> Result<JsValue, JsValue> {
    self.call1(&JsValue::undefined(), arg)
  }
}

impl<T, U, F> Callable<T, U> for Option<F>
where
  U: Default,
  F: Callable<T, U>,
{
  fn call(&self, arg: T) -> U {
    self.as_ref().map(|f| f.call(arg)).unwrap_or_default()
  }
}
