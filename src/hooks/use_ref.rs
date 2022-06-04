use crate::{callback::Void, react_bindings, Persisted, PersistedOrigin};
use js_sys::Reflect;
use std::{
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  mem::ManuallyDrop,
  rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsValue, UnwrapThrowExt};

/// Allows access to the underlying data persisted with [`use_ref()`].
///
/// The rules of borrowing will be enforced at runtime through a [`RefCell`].
#[derive(Debug)]
pub struct RefContainer<T> {
  inner: Rc<RefCell<T>>,
  js_ref: JsValue,
}

impl<T: 'static> RefContainer<T> {
  /// Returns a reference to the underlying data.
  pub fn current(&self) -> Ref<'_, T> {
    self.inner.borrow()
  }

  /// Returns a mutable reference to the underlying data.
  pub fn current_mut(&mut self) -> RefMut<'_, T> {
    self.inner.borrow_mut()
  }

  /// Sets the underlying data to the given value.
  pub fn set_current(&mut self, value: T) {
    *self.current_mut() = value;
  }

  /// Converts a JS value into a `RefContainer`.
  ///
  /// # Safety
  ///
  /// The following assumptions must hold:
  ///
  /// - The JS value has been obtained by creating a [`RefContainer`] by
  ///   [`use_ref()`] and converting it into [`JsValue`].
  /// - The React component owning the [`RefContainer`] hasn't been unmounted.
  ///
  /// Otherwise this might lead to memory problems.
  pub unsafe fn try_from_js(
    js_value: &JsValue,
  ) -> Result<RefContainer<T>, JsValue> {
    let ptr =
      react_bindings::cast_to_usize(&Reflect::get(js_value, &"ptr".into())?)
        as *const RefCell<T>;

    // We're wrapping the value in `ManuallyDrop` since we do not want to drop
    // the inner value when this is goes out of scope.
    let inner = ManuallyDrop::new(Rc::from_raw(ptr));
    let result = RefContainer {
      inner: (*inner).clone(),
      js_ref: js_value.clone(),
    };

    Ok(result)
  }
}

impl<T> Persisted for RefContainer<T> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

impl<T> Clone for RefContainer<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
      js_ref: self.js_ref.clone(),
    }
  }
}

impl<T> AsRef<JsValue> for RefContainer<T> {
  fn as_ref(&self) -> &JsValue {
    &self.js_ref
  }
}

impl<T> From<RefContainer<T>> for JsValue {
  fn from(value: RefContainer<T>) -> Self {
    value.js_ref
  }
}

/// This is the main hook for persisting Rust data through the entire lifetime
/// of the component.
///
/// Whenever the component is unmounted by React, the data will also be dropped.
/// Keep in mind that the inner value of [`use_ref()`] can only be accessed in
/// Rust. If you need a ref to hold a DOM element (or a JS value in general),
/// use [`use_js_ref()`] instead.
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
  let js_ref = react_bindings::use_rust_ref(
    Closure::once(move |_: Void| Rc::into_raw(Rc::new(RefCell::new(init))))
      .as_ref(),
    &Closure::once_into_js(|unmounted: bool, ptr: usize| {
      if unmounted {
        let ptr = ptr as *const RefCell<T>;

        // A callback with `unmounted == true` can only be called once (look
        // at `react-bindings.js#useRustRef`), so a double-free cannot happen!
        drop(unsafe { Rc::from_raw(ptr) });
      }
    }),
  );

  unsafe { RefContainer::try_from_js(&js_ref).unwrap_throw() }
}
