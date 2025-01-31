use std::{cell::RefCell, fmt::Debug, ops::DerefMut, rc::Rc};
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  describe::WasmDescribe,
  prelude::Closure,
  JsValue,
};

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

pub struct Callback<T, U = ()> {
  closure: Rc<RefCell<Box<dyn FnMut(T) -> U>>>,
  js: Rc<RefCell<Option<JsValue>>>,
}

impl<T, U> Callback<T, U>
where
  T: 'static,
  U: 'static,
{
  pub fn new(f: impl FnMut(T) -> U + 'static) -> Self {
    Self {
      closure: Rc::new(RefCell::new(Box::new(f))),
      js: Rc::new(RefCell::new(None)),
    }
  }

  pub fn call(&self, arg: T) -> U {
    (self.closure.borrow_mut())(arg)
  }

  pub fn to_closure(&self) -> impl FnMut(T) -> U + 'static {
    let cb = self.clone();
    move |arg| cb.call(arg)
  }

  /// Returns a new [`Callback`] by prepending the given closure to the callback.
  pub fn premap<V>(&self, mut f: impl FnMut(V) -> T + 'static) -> Callback<V, U>
  where
    V: 'static,
  {
    let cb = self.clone();
    Callback::new(move |v| cb.call(f(v)))
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
    Callback::new(move |t| f(cb.call(t)))
  }
}

impl<T, U> Debug for Callback<T, U> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Callback").finish()
  }
}

impl<T, U> Clone for Callback<T, U> {
  fn clone(&self) -> Self {
    Self {
      closure: self.closure.clone(),
      js: self.js.clone(),
    }
  }
}

impl<T, U> From<Callback<T, U>> for JsValue
where
  T: FromWasmAbi + 'static,
  U: IntoWasmAbi + 'static,
{
  fn from(value: Callback<T, U>) -> Self {
    let mut result = value.js.borrow_mut();
    let result = result.deref_mut().get_or_insert_with({
      let value = value.clone();
      move || {
        Closure::<dyn FnMut(T) -> U>::new(move |arg| value.call(arg))
          .into_js_value()
      }
    });

    result.clone()
  }
}

impl<T: 'static> Callback<T> {
  /// Returns a new [`Callback`] that does nothing.
  pub fn noop() -> Self {
    Callback::default()
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

impl<T, U> PartialEq for Callback<T, U>
where
  T: 'static,
  U: 'static,
{
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.closure, &other.closure)
  }
}

impl<T, U> Eq for Callback<T, U>
where
  T: 'static,
  U: 'static,
{
}
