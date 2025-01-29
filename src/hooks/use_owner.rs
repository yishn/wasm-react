use crate::react_bindings;
use generational_box::{AnyStorage, Owner, UnsyncStorage};
use std::ops::Deref;
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

#[wasm_bindgen(js_name = __WasmReact_OwnerContainer)]
#[derive(Clone)]
pub struct OwnerContainer(Owner);

impl OwnerContainer {
  pub fn new() -> Self {
    Self(UnsyncStorage::owner())
  }
}

impl Deref for OwnerContainer {
  type Target = Owner;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub(crate) fn use_owner_setup() {
  react_bindings::use_owner_setup(&|| OwnerContainer::new());
}

pub(super) fn use_owner() -> OwnerContainer {
  let mut result = None;

  react_bindings::use_owner(&mut |owner_container| {
    result = Some(owner_container.clone());
  });

  result.unwrap_throw()
}
