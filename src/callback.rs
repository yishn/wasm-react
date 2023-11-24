use std::{
  cell::{Ref, RefCell},
  fmt::Debug,
  rc::Rc,
};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  describe::WasmDescribe,
  prelude::Closure,
  JsValue, UnwrapThrowExt,
};

/// A zero-sized helper struct to simulate a JS-interoperable [`Callback`] with no input
/// arguments.
///
/// ```
/// # use wasm_react::*;
/// # fn f() {
/// let cb: Callback<Void, usize> = Callback::new(|Void| 5);
/// # }
/// ```
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Void;

impl WasmDescribe for Void {
  fn describe() {
    JsValue::describe()
  }
}

impl IntoWasmAbi for Void {
  type Abi = <JsValue as IntoWasmAbi>::Abi;

  fn into_abi(self) -> Self::Abi {
    JsValue::undefined().into_abi()
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

/// This is a simplified, reference-counted wrapper around an [`FnMut(T) -> U`](FnMut)
/// Rust closure that may be called from JS when `T` and `U` allow.
///
/// You can also use the [`clones!`](crate::clones!) helper macro to
/// clone-capture the environment more ergonomically.
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
    let callback = self.clone();
    move |arg| callback.call(arg)
  }

  /// Calls the callback with the given argument.
  pub fn call(&self, arg: T) -> U {
    let mut f = self.closure.borrow_mut();
    f(arg)
  }

  /// Returns a new [`Callback`] by prepending the given closure to the callback.
  pub fn premap<V>(&self, mut f: impl FnMut(V) -> T + 'static) -> Callback<V, U>
  where
    V: 'static,
  {
    let cb = self.clone();

    Callback::new(move |v| {
      let t = f(v);
      cb.call(t)
    })
  }

  /// Returns a new [`Callback`] by appending the given closure to the callback.
  pub fn postmap<V>(
    &self,
    mut f: impl FnMut(U) -> V + 'static,
  ) -> Callback<T, V>
  where
    V: 'static,
  {
    let cb = self.clone();

    Callback::new(move |t| {
      let u = cb.call(t);
      f(u)
    })
  }

  /// Returns a reference to `JsValue` of the callback.
  pub fn as_js(&self) -> Ref<'_, JsValue>
  where
    T: FromWasmAbi,
    U: IntoWasmAbi,
  {
    {
      self.js.borrow_mut().get_or_insert_with(|| {
        Closure::new({
          let closure = self.closure.clone();

          move |arg| {
            let mut f = closure.borrow_mut();
            f(arg)
          }
        })
      });
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

impl<F, T, U> From<F> for Callback<T, U>
where
  F: FnMut(T) -> U + 'static,
  T: 'static,
  U: 'static,
{
  fn from(value: F) -> Self {
    Self::new(value)
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
