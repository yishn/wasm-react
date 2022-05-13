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

enum CallbackType<T, U> {
  Multiple(Rc<RefCell<dyn FnMut(T) -> U>>),
  Once(Rc<RefCell<Option<Box<dyn FnOnce(T) -> U>>>>),
}

impl<T, U> PartialEq for CallbackType<T, U> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Multiple(x), Self::Multiple(y)) => Rc::ptr_eq(x, y),
      (Self::Once(x), Self::Once(y)) => Rc::ptr_eq(x, y),
      _ => false,
    }
  }
}

impl<T, U> Eq for CallbackType<T, U> {}

impl<T, U> Clone for CallbackType<T, U> {
  fn clone(&self) -> Self {
    match self {
      Self::Multiple(x) => Self::Multiple(x.clone()),
      Self::Once(x) => Self::Once(x.clone()),
    }
  }
}

/// A smart pointer that derefs to `JsValue`.
pub struct CallbackJsRef<'a, T, U>(Ref<'a, Option<Closure<dyn FnMut(T) -> U>>>);

impl<'a, T, U> Clone for CallbackJsRef<'a, T, U> {
  fn clone(&self) -> Self {
    Self(Ref::clone(&self.0))
  }
}

impl<'a, T, U> Deref for CallbackJsRef<'a, T, U>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
{
  type Target = JsValue;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref().unwrap_throw().as_ref()
  }
}

/// This is a simplified, reference-counted wrapper around a Rust closure that
/// may be called from JS when `T` and `U` allow.
///
/// It only supports closures with exactly one input argument and some return
/// value. Memory management is handled by Rust. Whenever Rust drops all clones
/// of the [`Callback`], the closure will be dropped and the function cannot be
/// called from JS anymore.
///
/// Use [`Void`] to simulate a callback with no arguments.
pub struct Callback<T, U = ()> {
  closure: CallbackType<T, U>,
  js: Rc<RefCell<Option<Closure<dyn FnMut(T) -> U>>>>,
}

impl<T, U> Callback<T, U> {
  /// Creates a new [`Callback`] from an [`Fn`].
  pub fn new(f: impl FnMut(T) -> U + 'static) -> Self {
    Self {
      closure: CallbackType::Multiple(Rc::new(RefCell::new(f))),
      js: Rc::new(RefCell::new(None)),
    }
  }

  /// Creates a new [`Callback`] from an [`FnOnce`] that can only be called once.
  /// Trying to call it more than once will result in a **panic**.
  pub fn once(f: impl FnOnce(T) -> U + 'static) -> Self {
    Self {
      closure: CallbackType::Once(Rc::new(RefCell::new(Some(Box::new(f))))),
      js: Rc::new(RefCell::new(None)),
    }
  }
}

impl<T> Callback<T> {
  /// Returns a new [`Callback`] that does nothing.
  pub fn noop() -> Self {
    Callback::new(|_: T| ())
  }
}

impl<T, U> Callback<T, U>
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
{
  /// Returns a reference to `JsValue` of the callback.
  pub fn as_js(&self) -> CallbackJsRef<'_, T, U> {
    if self.js.borrow().is_none() {
      *self.js.borrow_mut() = Some(match &self.closure {
        CallbackType::Multiple(x) => Closure::wrap(Box::new({
          let x = x.clone();

          move |arg| {
            let mut f = x.borrow_mut();
            f(arg)
          }
        })),
        CallbackType::Once(x) => {
          let x = x.clone();

          Closure::once(move |arg| {
            let f = x.borrow_mut().take().unwrap_throw();
            f(arg)
          })
        }
      });
    }

    CallbackJsRef(self.js.borrow())
  }
}

impl<T> Default for Callback<T, ()>
where
  T: FromWasmAbi + 'static,
{
  fn default() -> Self {
    Self::new(|_| ())
  }
}

impl<T, U> Debug for Callback<T, U> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Callback(|_| { ... })")
  }
}

impl<T, U> PartialEq for Callback<T, U> {
  fn eq(&self, other: &Self) -> bool {
    self.closure == other.closure && Rc::ptr_eq(&self.js, &other.js)
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
    f.write_str("PersistedCallback(|_| { ... })")
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

impl<T, U> Persisted for PersistedCallback<T, U> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

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

impl<T, U> Callable<T, U> for Callback<T, U> {
  fn call(&self, arg: T) -> U {
    match &self.closure {
      CallbackType::Multiple(x) => {
        let mut f = x.borrow_mut();
        f(arg)
      }
      CallbackType::Once(x) => {
        let f = x.borrow_mut().take().unwrap_throw();
        f(arg)
      }
    }
  }
}

impl<T, U> Callable<T, U> for PersistedCallback<T, U> {
  fn call(&self, arg: T) -> U {
    self.deref().call(arg)
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
