use crate::{callback::Void, react_bindings, Persisted, PersistedOrigin};
use std::{
  any::Any,
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  rc::Rc,
};
use wasm_bindgen::prelude::*;

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_RefContainerValue)]
#[derive(Debug, Clone)]
pub struct RefContainerValue(pub(crate) Rc<dyn Any>);

impl RefContainerValue {
  pub fn value<T: 'static>(&self) -> Result<Rc<T>, Rc<dyn Any>> {
    Rc::downcast::<T>(self.0.clone())
  }
}

/// Allows access to the underlying data persisted with [`use_ref()`].
///
/// # Panics
///
/// The rules of borrowing will be enforced at runtime through a [`RefCell`],
/// therefore the methods [`RefContainer::current()`],
/// [`RefContainer::current_mut()`], and [`RefContainer::set_current()`] may
/// panic accordingly.
#[derive(Debug)]
pub struct RefContainer<T>(Rc<RefCell<T>>);

impl<T: 'static> RefContainer<T> {
  /// Returns a reference to the underlying data.
  ///
  /// # Panics
  ///
  /// Panics if the underlying data is currently mutably borrowed.
  pub fn current(&self) -> Ref<'_, T> {
    self.0.borrow()
  }

  /// Returns a mutable reference to the underlying data.
  ///
  /// # Panics
  ///
  /// Panics if the underlying data is currently borrowed.
  pub fn current_mut(&mut self) -> RefMut<'_, T> {
    self.0.borrow_mut()
  }

  /// Sets the underlying data to the given value.
  ///
  /// # Panics
  ///
  /// Panics if the underlying data is currently borrowed.
  pub fn set_current(&mut self, value: T) {
    *self.current_mut() = value;
  }
}

impl<T: 'static> Persisted for RefContainer<T> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

impl<T> Clone for RefContainer<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

/// This is the main hook for persisting Rust data through the entire lifetime
/// of the component.
///
/// Whenever the component is unmounted by React, the data will also be dropped.
/// Keep in mind that the inner value of [`use_ref()`] can only be accessed in
/// Rust. If you need a ref to hold a DOM element (or a JS value in general),
/// use [`use_js_ref()`](crate::hooks::use_js_ref()) instead.
///
/// The component will not rerender when you mutate the underlying data. If you
/// want that, use [`use_state()`](crate::hooks::use_state()) instead.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// # struct MyData { value: &'static str };
/// # struct MyComponent { value: &'static str };
/// #
/// impl Component for MyComponent {
///   fn render(&self) -> VNode {
///     let ref_container = use_ref(MyData {
///       value: "Hello World!"
///     });
///
///     use_effect({
///       let value = self.value;
///       let mut ref_container = ref_container.clone();
///
///       move || {
///         ref_container.current_mut().value = value;
///         || ()
///       }
///     }, Deps::some(self.value));
///
///     h!(div).build(c![
///       ref_container.current().value
///     ])
///   }
/// }
/// ```
pub fn use_ref<T: 'static>(init: T) -> RefContainer<T> {
  let mut value = None;

  react_bindings::use_rust_ref(
    Closure::once(move |_: Void| {
      RefContainerValue(Rc::new(RefCell::new(init)))
    })
    .as_ref(),
    &mut |ref_container_value| {
      value = Some(
        ref_container_value
          .value::<RefCell<T>>()
          .expect("mismatched ref container type"),
      );
    },
  );

  RefContainer(value.expect("callback was not called"))
}
