#![cfg(feature = "web-sys")]

use super::{HtmlTag, H};
use paste::paste;
use wasm_bindgen::intern;
#[cfg(feature = "web-sys")]
use web_sys::{
  AnimationEvent, DragEvent, Event, FocusEvent, KeyboardEvent, MouseEvent,
  PointerEvent, TransitionEvent, UiEvent, WheelEvent,
};

use crate::Callback;

macro_rules! impl_event {
  { $( $Event:ident => $E:ty; )* } => {
    $(
      paste! {
        #[allow(missing_docs)]
        pub fn [<on_ $Event:lower>](self, f: &Callback<$E>) -> Self {
          self.attr_callback(intern(stringify!([<on $Event>])), f)
        }

        #[allow(missing_docs)]
        pub fn [<on_ $Event:lower _capture>](self, f: &Callback<$E>) -> Self {
          self.attr_callback(intern(stringify!([<on $Event Capture>])), f)
        }
      }
    )*
  };
}

/// Provides auto-completion for DOM events on [`H`].
#[cfg(feature = "web-sys")]
impl H<HtmlTag<'_>> {
  impl_event! {
    Focus => FocusEvent;
    Blur => FocusEvent;

    Change => Event;
    BeforeInput => Event;
    Input => Event;
    Reset => Event;
    Submit => Event;
    Invalid => Event;
    Select => UiEvent;

    Load => Event;

    KeyDown => KeyboardEvent;
    KeyPress => KeyboardEvent;
    KeyUp => KeyboardEvent;

    Abort => Event;
    CanPlay => Event;
    CanPlayThrough => Event;
    DurationChange => Event;
    Emptied => Event;
    Encrypted => Event;
    Ended => Event;
    LoadedData => Event;
    LoadedMetadata => Event;
    LoadStart => Event;
    Pause => Event;
    Play => Event;
    Playing => Event;
    Progress => Event;
    RateChange => Event;
    Seeked => Event;
    Seeking => Event;
    Stalled => Event;
    Suspend => Event;
    TimeUpdate => Event;
    VolumeChange => Event;
    Waiting => Event;

    AuxClick => MouseEvent;
    Click => MouseEvent;
    ContextMenu => MouseEvent;
    DoubleClick => MouseEvent;
    MouseDown => MouseEvent;
    MouseEnter => MouseEvent;
    MouseLeave => MouseEvent;
    MouseMove => MouseEvent;
    MouseOut => MouseEvent;
    MouseOver => MouseEvent;
    MouseUp => MouseEvent;

    PointerDown => PointerEvent;
    PointerMove => PointerEvent;
    PointerUp => PointerEvent;
    PointerCancel => PointerEvent;
    PointerEnter => PointerEvent;
    PointerLeave => PointerEvent;
    PointerOver => PointerEvent;
    PointerOut => PointerEvent;
    GotPointerCapture => PointerEvent;
    LostPointerCapture => PointerEvent;

    Drag => DragEvent;
    DragEnd => DragEvent;
    DragEnter => DragEvent;
    DragExit => DragEvent;
    DragLeave => DragEvent;
    DragOver => DragEvent;
    DragStart => DragEvent;
    Drop => DragEvent;

    Scroll => UiEvent;
    Wheel => WheelEvent;

    AnimationStart => AnimationEvent;
    AnimationEnd => AnimationEvent;
    AnimationIteration => AnimationEvent;
    TransitionEnd => TransitionEvent;
  }
}
