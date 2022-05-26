use crate::{callback::Void, react_bindings, Persisted, PersistedOrigin};
use js_sys::Reflect;
use std::{
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  marker::PhantomData,
  mem::ManuallyDrop,
  rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

/// Allows access to the underlying data persisted with [`use_ref()`].
///
/// The rules of borrowing will be enforced at runtime through a [`RefCell`].
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
    &Closure::once_into_js(move |unmounted: bool, ptr: usize| {
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

/// Allows access to the underlying JS data persisted with [`use_js_ref()`].
pub struct JsRefContainer<T>(JsValue, PhantomData<T>);

impl<T: JsCast> JsRefContainer<T> {
  /// Returns the underlying typed JS data.
  pub fn current(&self) -> Option<T> {
    self.current_untyped().dyn_into::<T>().ok()
  }

  /// Returns the underlying JS data as [`JsValue`].
  pub fn current_untyped(&self) -> JsValue {
    Reflect::get(&self.0, &"current".into()).unwrap_throw()
  }

  /// Sets the underlying JS data.
  pub fn set_current(&self, value: Option<&T>) {
    Reflect::set(
      &self.0,
      &"current".into(),
      value.map(|t| t.as_ref()).unwrap_or(&JsValue::null()),
    )
    .unwrap_throw();
  }
}

impl<T> Persisted for JsRefContainer<T> {
  fn ptr(&self) -> PersistedOrigin {
    PersistedOrigin
  }
}

impl<T> Debug for JsRefContainer<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("JsRefContainer").field(&self.0).finish()
  }
}

impl<T> Clone for JsRefContainer<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), PhantomData)
  }
}

impl<T> AsRef<JsValue> for JsRefContainer<T> {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}

impl<T> From<JsRefContainer<T>> for JsValue {
  fn from(value: JsRefContainer<T>) -> Self {
    value.0
  }
}

/// This hook can persist JS data through the entire lifetime of the component.
///
/// Use this if you need JS to set the ref value. If you only need to mutate the
/// data from Rust, use [`use_ref()`] instead.
///
/// # Example
///
/// ```
/// # use wasm_react::{*, hooks::*};
/// # struct MyComponent;
/// impl Component for MyComponent {
///   fn render(&self) -> VNode {
///     let input_element = use_js_ref(None);
///
///     h!(div)
///       .build(c![
///         h!(input)
///           .ref_container(&input_element)
///           .html_type("text")
///           .build(c![])
///       ])
///   }
/// }
/// ```
pub fn use_js_ref<T: JsCast>(init: Option<T>) -> JsRefContainer<T> {
  let ref_container = react_bindings::use_ref(
    &init.map(|init| init.into()).unwrap_or(JsValue::null()),
  );

  JsRefContainer(ref_container, PhantomData)
}
