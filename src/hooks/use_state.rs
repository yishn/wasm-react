use crate::{react, Callback};
use js_sys::Function;
use std::ops::Deref;
use wasm_bindgen::{JsCast, JsValue};

pub struct UseState<T>(*mut T, Function);

impl<T> Clone for UseState<T> {
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

impl<T> Deref for UseState<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    Box::leak(unsafe { Box::from_raw(self.0) })
  }
}

impl<T: 'static> UseState<T> {
  pub fn update(&self, mutator: impl Fn(&mut T) + 'static) {
    let ptr = self.0;

    self
      .1
      .call1(
        &JsValue::undefined(),
        &Callback::new(move |_: JsValue| {
          let state = Box::leak(unsafe { Box::from_raw(ptr) });
          mutator(state);
        })
        .into(),
      )
      .ok();
  }
}

pub fn use_state<T: 'static>(value: impl Fn() -> T) -> UseState<T> {
  // The lifetime of the state (`T`) is completely managed by the React
  // component lifetime, meaning whenever the component is unmounted by React,
  // the state will also be dropped.

  let result = react::use_rust_state(
    &|| Box::into_raw(Box::new(value())) as usize,
    // This callback will be called when the component unmounts
    Callback::new(|ptr: usize| unsafe {
      drop(Box::from_raw(ptr as *mut T));
    })
    .into(),
  );
  let update_state = result.get(1).dyn_into::<Function>().unwrap();
  let ptr = react::cast_into_usize(result.get(0)) as *mut T;

  UseState(ptr, update_state)
}
