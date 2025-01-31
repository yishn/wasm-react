use super::get_owner;
use crate::react_bindings;
use generational_box::{GenerationalBox, GenerationalRef, GenerationalRefMut};
use std::{
  any::Any,
  fmt::Debug,
  marker::PhantomData,
  ops::{Deref, DerefMut},
};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_AnyRefContainer)]
#[derive(Debug, Clone, Copy)]
pub struct AnyRefContainer(GenerationalBox<Box<dyn Any>>);

pub struct RefContainerRef<T>(
  GenerationalRef<std::cell::Ref<'static, Box<dyn Any>>>,
  PhantomData<T>,
);

impl<T: 'static> Deref for RefContainerRef<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.downcast_ref().unwrap_throw()
  }
}

pub struct RefContainerMut<T>(
  GenerationalRefMut<std::cell::RefMut<'static, Box<dyn Any>>>,
  PhantomData<T>,
);

impl<T: 'static> Deref for RefContainerMut<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.downcast_ref().unwrap_throw()
  }
}

impl<T: 'static> DerefMut for RefContainerMut<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0.downcast_mut().unwrap_throw()
  }
}

pub struct RefContainer<T>(AnyRefContainer, PhantomData<T>);

impl<T: 'static> RefContainer<T> {
  pub fn current(self) -> RefContainerRef<T> {
    RefContainerRef(self.0 .0.read(), PhantomData)
  }

  pub fn current_mut(self) -> RefContainerMut<T> {
    RefContainerMut(self.0 .0.write(), PhantomData)
  }

  pub fn set_current(self, value: T) {
    *self.current_mut() = value;
  }
}

impl<T> Debug for RefContainer<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("RefContainer")
      .field(&self.0)
      .field(&self.1)
      .finish()
  }
}

impl<T> Clone for RefContainer<T> {
  fn clone(&self) -> Self {
    Self(self.0, PhantomData)
  }
}

impl<T> Copy for RefContainer<T> {}

pub fn use_ref<T: 'static>(init: impl FnOnce() -> T) -> RefContainer<T> {
  let owner = get_owner();
  let mut result = None;
  let mut init = Some(init);

  react_bindings::use_ref(
    &mut move || {
      init
        .take()
        .map(|init| AnyRefContainer(owner.insert(Box::new(init()))))
        .unwrap_throw()
    },
    &mut |container| {
      result = Some(RefContainer(*container, PhantomData));
    },
  );

  result.unwrap_throw()
}
