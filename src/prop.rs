use crate::hooks::{
  get_tmp_owner, DeferredValue, DeferredValueRef, Memo, MemoRef, RefContainer,
  RefContainerRef, State, StateRef,
};
use generational_box::{GenerationalBox, GenerationalRef};
use std::{cell, fmt::Debug, ops::Deref};

#[doc(hidden)]
pub trait PropContainer<T> {
  type Ref: Deref<Target = T>;

  fn get(self) -> Self::Ref;
}

impl<T: 'static> PropContainer<T> for GenerationalBox<T> {
  type Ref = GenerationalRef<cell::Ref<'static, T>>;

  fn get(self) -> Self::Ref {
    self.read()
  }
}

impl<T: 'static> PropContainer<T> for RefContainer<T> {
  type Ref = RefContainerRef<T>;

  fn get(self) -> Self::Ref {
    self.current()
  }
}

impl<T: 'static> PropContainer<T> for State<T> {
  type Ref = StateRef<T>;

  fn get(self) -> Self::Ref {
    State::get(self)
  }
}

impl<T: 'static> PropContainer<T> for Memo<T> {
  type Ref = MemoRef<T>;

  fn get(self) -> Self::Ref {
    Memo::get(self)
  }
}

impl<T: 'static> PropContainer<T> for DeferredValue<T> {
  type Ref = DeferredValueRef<T>;

  fn get(self) -> Self::Ref {
    DeferredValue::get(self)
  }
}

macro_rules! define_prop {
  { $( $Variant:ident $(,)? )* } => {
    #[non_exhaustive]
    pub enum Prop<T> {
      $(
        $Variant($Variant<T>),
      )*
    }

    impl<T: 'static> Prop<T> {
      pub fn get(self) -> PropRef<T> {
        match self {
          $( Self::$Variant(inner) => PropRef::$Variant(inner.get()), )*
        }
      }
    }

    impl<T> Debug for Prop<T>
    where
      T: Debug + 'static,
    {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(
            Self::$Variant(_) =>
              f.debug_tuple(stringify!(Prop::$Variant))
                .field(&*self.get())
                .finish(),
          )*
        }
      }
    }

    impl<T> Clone for Prop<T> {
      fn clone(&self) -> Self {
        match self {
          $( Self::$Variant(inner) => Self::$Variant(inner.clone()), )*
        }
      }
    }

    impl<T> Copy for Prop<T> {}

    $(
      impl<T: 'static> From<$Variant<T>> for Prop<T> {
        fn from(value: $Variant<T>) -> Self {
          Self::$Variant(value)
        }
      }
    )*

    #[non_exhaustive]
    pub enum PropRef<T: 'static> {
      $(
        $Variant(<$Variant<T> as PropContainer<T>>::Ref),
      )*
    }

    impl<T> Deref for PropRef<T> {
      type Target = T;

      fn deref(&self) -> &Self::Target {
        match self {
          $( Self::$Variant(inner) => &inner, )*
        }
      }
    }
  };
}

define_prop! {
  GenerationalBox,
  RefContainer,
  State,
  Memo,
  DeferredValue,
}

impl<T: 'static> From<T> for Prop<T> {
  fn from(value: T) -> Self {
    let owner = get_tmp_owner();
    Prop::GenerationalBox(owner.insert(value))
  }
}

impl<T: PartialEq + 'static> PartialEq for Prop<T> {
  fn eq(&self, other: &Self) -> bool {
    T::eq(&self.get(), &other.get())
  }
}

impl<T: PartialEq + 'static> PartialEq<T> for Prop<T> {
  fn eq(&self, other: &T) -> bool {
    T::eq(&self.get(), other)
  }
}
