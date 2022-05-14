use crate::{callback::Void, react_bindings, Persisted, PersistedOrigin};
use js_sys::Reflect;
use std::{
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  marker::PhantomData,
  rc::Rc,
};
use wasm_bindgen::{
  prelude::Closure, throw_val, JsCast, JsError, JsValue, UnwrapThrowExt,
};

/// Allows access to the underlying data persisted with [`use_ref()`].
///
/// The rules of borrowing will be enforced at runtime through a [`RefCell`].
///
/// When the component unmounts, the underlying data is dropped. After that,
/// trying to access the data will result in a **panic**.
pub struct RefContainer<T> {
  ptr: Rc<RefCell<*mut T>>,
  js_ref: JsValue,
}

impl<T: 'static> RefContainer<T> {
  fn check_dropped(&self) {
    // Memory safety: Only yield underlying data if data has not been dropped
    // already!

    let dropped = Reflect::get(&self.js_ref, &"dropped".into())
      .unwrap_throw()
      .as_bool()
      .unwrap_throw();

    if dropped {
      throw_val(
        JsError::new(
          "You're trying to use a hook on a component that has already been unmounted!",
        )
        .into(),
      );
    }
  }

  /// Returns a reference to the underlying data.
  pub fn current(&self) -> Ref<'_, T> {
    self.check_dropped();

    Ref::map(self.ptr.borrow(), |ptr| {
      Box::leak(unsafe { Box::from_raw(*ptr) })
    })
  }

  /// Returns a mutable reference to the underlying data.
  pub fn current_mut(&mut self) -> RefMut<'_, T> {
    self.check_dropped();

    RefMut::map(self.ptr.borrow_mut(), |ptr| {
      Box::leak(unsafe { Box::from_raw(*ptr) })
    })
  }

  /// Sets the underlying data to the given value.
  pub fn set_current(&mut self, value: T) {
    *self.current_mut() = value;
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
      ptr: self.ptr.clone(),
      js_ref: self.js_ref.clone(),
    }
  }
}

impl<T> AsRef<JsValue> for RefContainer<T> {
  fn as_ref(&self) -> &JsValue {
    &self.js_ref
  }
}

impl<T> TryFrom<JsValue> for RefContainer<T> {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(RefContainer {
      ptr: Rc::new(RefCell::new(react_bindings::cast_to_usize(&Reflect::get(
        &value,
        &"ptr".into(),
      )?) as *mut T)),
      js_ref: value,
    })
  }
}

/// This is the main hook for persisting Rust data through the entire lifetime
/// of the component.
///
/// Whenever the component is unmounted by React, the data will also be dropped.
/// Keep in mind that [`use_ref()`] can only be mutated in Rust. If you need a
/// ref to hold a DOM element, use [`use_js_ref()`] instead.
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
    Closure::once(move |_: Void| Box::into_raw(Box::new(init))).as_ref(),
    &Closure::once_into_js(
      move |unmounted: bool, ptr: usize, js_ref: JsValue| {
        if unmounted {
          let ptr = ptr as *mut T;

          // A callback with `unmounted == true` can only be called once (look
          // at `react-bindings.js#useRustRef`), so a double-free cannot happen!
          drop(unsafe { Box::from_raw(ptr) });

          // By setting `dropped` to `true`, we're signalling that the
          // underlying data has already been dropped and that it is not safe
          // for `RefContainer` to access it anymore.
          Reflect::set(&js_ref, &"dropped".into(), &JsValue::TRUE)
            .unwrap_throw();
        }
      },
    ),
  );

  RefContainer::try_from(js_ref).unwrap_throw()
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
