use js_sys::{Array, JsString};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct VNode(pub(crate) JsValue);

impl Default for VNode {
  fn default() -> Self {
    VNode(JsValue::null())
  }
}

impl AsRef<JsValue> for VNode {
  fn as_ref(&self) -> &JsValue {
    &self.0
  }
}

impl From<VNode> for JsValue {
  fn from(vnode: VNode) -> JsValue {
    vnode.0
  }
}

impl<T: Into<VNode>> From<Option<T>> for VNode {
  fn from(value: Option<T>) -> Self {
    value.map(|value| value.into()).unwrap_or_default()
  }
}

impl FromIterator<VNode> for VNode {
  fn from_iter<I: IntoIterator<Item = VNode>>(iter: I) -> Self {
    let vnodes = iter.into_iter().collect::<Array>();
    VNode(vnodes.into())
  }
}

macro_rules! impl_into_vnode {
  { $( $T:ty ),* $(,)? } => {
    $(
      impl From<$T> for VNode {
        fn from(value: $T) -> Self {
          VNode(value.into())
        }
      }
    )*
  };
}

// Implement `Into<VNode>` for as many primitive types as possible
impl_into_vnode! {
  &str, String, JsString,
  f32, f64,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
}

impl From<()> for VNode {
  fn from(_: ()) -> Self {
    VNode::default()
  }
}

macro_rules! impl_into_vnode_for_tuples {
  (@impl) => {};
  (@impl $( $x:ident ),+) => {
    #[allow(non_snake_case)]
    impl<$( $x, )+> From<($( $x, )+)> for VNode
    where $( $x: Into<VNode>, )+
    {
      fn from(($( $x, )+): ($( $x, )+)) -> VNode {
        let result = Array::new();
        $( result.push(&$x.into().into()); )+
        VNode(result.into())
      }
    }

    impl_into_vnode_for_tuples!(@next $( $x ),+);
  };
  (@next $first:ident) => {};
  (@next $first:ident, $( $tt:tt )*) => {
    impl_into_vnode_for_tuples!(@impl $( $tt )*);
  };
  ($( $x:ident ),*) => {
    impl_into_vnode_for_tuples!(@impl $( $x ),*);
  }
}

impl_into_vnode_for_tuples!(
  A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
