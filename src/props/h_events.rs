use super::H;
use crate::Callback;
use web_sys::{
  AnimationEvent, DragEvent, Event, FocusEvent, KeyboardEvent, MouseEvent,
  PointerEvent, TransitionEvent, UiEvent, WheelEvent,
};

macro_rules! impl_event {
  { $( $on_event:ident, $on_event_str:expr, $E:ty; )* } => {
    $(
      pub fn $on_event(self, f: Callback<$E>) -> Self {
        self.attr_callback($on_event_str, f)
      }
    )*
  };
}

/// Provides auto-completion for DOM events on [`H`].
impl<'a> H<'a> {
  impl_event! {
    on_focus, "onFocus", FocusEvent;
    on_focus_capture, "onFocusCapture", FocusEvent;
    on_blur, "onBlur", FocusEvent;
    on_blur_capture, "onBlurCapture", FocusEvent;

    on_change, "onChange", Event;
    on_change_capture, "onChangeCapture", Event;
    on_beforeinput, "onBeforeInput", Event;
    on_beforeinput_capture, "onBeforeInputCapture", Event;
    on_input, "onInput", Event;
    on_input_capture, "onInputCapture", Event;
    on_reset, "onReset", Event;
    on_reset_capture, "onResetCapture", Event;
    on_submit, "onSubmit", Event;
    on_submit_capture, "onSubmitCapture", Event;
    on_invalid, "onInvalid", Event;
    on_invalid_capture, "onInvalidCapture", Event;
    on_select, "onSelect", UiEvent;
    on_select_capture, "onSelectCapture", UiEvent;

    on_load, "onLoad", Event;
    on_load_capture, "onLoadCapture", Event;

    on_keydown, "onKeyDown", KeyboardEvent;
    on_keydown_capture, "onKeyDownCapture", KeyboardEvent;
    on_keypress, "onKeyPress", KeyboardEvent;
    on_keypress_capture, "onKeyPressCapture", KeyboardEvent;
    on_keyup, "onKeyUp", KeyboardEvent;
    on_keyup_capture, "onKeyUpCapture", KeyboardEvent;

    on_abort, "onAbort", Event;
    on_abort_capture, "onAbortCapture", Event;
    on_canplay, "onCanPlay", Event;
    on_canplay_capture, "onCanPlayCapture", Event;
    on_canplay_through, "onCanPlayThrough", Event;
    on_canplay_through_capture, "onCanPlayThroughCapture", Event;
    on_duration_change, "onDurationChange", Event;
    on_duration_change_capture, "onDurationChangeCapture", Event;
    on_emptied, "onEmptied", Event;
    on_emptied_capture, "onEmptiedCapture", Event;
    on_encrypted, "onEncrypted", Event;
    on_encrypted_capture, "onEncryptedCapture", Event;
    on_ended, "onEnded", Event;
    on_ended_capture, "onEndedCapture", Event;
    on_loadeddata, "onLoadedData", Event;
    on_loadeddata_capture, "onLoadedDataCapture", Event;
    on_loadedmetadata, "onLoadedMetadata", Event;
    on_loadedmetadata_capture, "onLoadedMetadataCapture", Event;
    on_loadstart, "onLoadStart", Event;
    on_loadstart_capture, "onLoadStartCapture", Event;
    on_pause, "onPause", Event;
    on_pause_capture, "onPauseCapture", Event;
    on_play, "onPlay", Event;
    on_play_capture, "onPlayCapture", Event;
    on_playing, "onPlaying", Event;
    on_playing_capture, "onPlayingCapture", Event;
    on_progress, "onProgress", Event;
    on_progress_capture, "onProgressCapture", Event;
    on_ratechange, "onRateChange", Event;
    on_ratechange_capture, "onRateChangeCapture", Event;
    on_seeked, "onSeeked", Event;
    on_seeked_capture, "onSeekedCapture", Event;
    on_seeking, "onSeeking", Event;
    on_seeking_capture, "onSeekingCapture", Event;
    on_stalled, "onStalled", Event;
    on_stalled_capture, "onStalledCapture", Event;
    on_suspend, "onSuspend", Event;
    on_suspend_capture, "onSuspendCapture", Event;
    on_timeupdate, "onTimeUpdate", Event;
    on_timeupdate_capture, "onTimeUpdateCapture", Event;
    on_volumechange, "onVolumeChange", Event;
    on_volumechange_capture, "onVolumeChangeCapture", Event;
    on_waiting, "onWaiting", Event;
    on_waiting_capture, "onWaitingCapture", Event;

    on_auxclick, "onAuxClick", MouseEvent;
    on_auxclick_capture, "onAuxClickCapture", MouseEvent;
    on_click, "onClick", MouseEvent;
    on_click_capture, "onClickCapture", MouseEvent;
    on_context_menu, "onContextMenu", MouseEvent;
    on_context_menu_capture, "onContextMenuCapture", MouseEvent;
    on_doubleclick, "onDoubleClick", MouseEvent;
    on_doubleclick_capture, "onDoubleClickCapture", MouseEvent;
    on_mousedown, "onMouseDown", MouseEvent;
    on_mousedown_capture, "onMouseDownCapture", MouseEvent;
    on_mouseenter, "onMouseEnter", MouseEvent;
    on_mouseleave, "onMouseLeave", MouseEvent;
    on_mousemove, "onMouseMove", MouseEvent;
    on_mousemove_capture, "onMouseMoveCapture", MouseEvent;
    on_mouseout, "onMouseOut", MouseEvent;
    on_mouseout_capture, "onMouseOutCapture", MouseEvent;
    on_mouseover, "onMouseOver", MouseEvent;
    on_mouseover_capture, "onMouseOverCapture", MouseEvent;
    on_mouseup, "onMouseUp", MouseEvent;
    on_mouseup_capture, "onMouseUpCapture", MouseEvent;

    on_pointerdown, "onPointerDown", PointerEvent;
    on_pointerdown_capture, "onPointerDownCapture", PointerEvent;
    on_pointermove, "onPointerMove", PointerEvent;
    on_pointermove_capture, "onPointerMoveCapture", PointerEvent;
    on_pointerup, "onPointerUp", PointerEvent;
    on_pointerup_capture, "onPointerUpCapture", PointerEvent;
    on_pointercancel, "onPointerCancel", PointerEvent;
    on_pointercancel_capture, "onPointerCancelCapture", PointerEvent;
    on_pointerenter, "onPointerEnter", PointerEvent;
    on_pointerenter_capture, "onPointerEnterCapture", PointerEvent;
    on_pointerleave, "onPointerLeave", PointerEvent;
    on_pointerleave_capture, "onPointerLeaveCapture", PointerEvent;
    on_pointerover, "onPointerOver", PointerEvent;
    on_pointerover_capture, "onPointerOverCapture", PointerEvent;
    on_pointerout, "onPointerOut", PointerEvent;
    on_pointerout_capture, "onPointerOutCapture", PointerEvent;
    on_gotpointer_capture, "onGotPointerCapture", PointerEvent;
    on_gotpointer_capture_capture, "onGotPointerCaptureCapture", PointerEvent;
    on_lostpointer_capture, "onLostPointerCapture", PointerEvent;
    on_lostpointer_capture_capture, "onLostPointerCaptureCapture", PointerEvent;

    on_drag, "onDrag", DragEvent;
    on_dragcapture, "onDragCapture", DragEvent;
    on_dragend, "onDragEnd", DragEvent;
    on_dragend_capture, "onDragEndCapture", DragEvent;
    on_dragenter, "onDragEnter", DragEvent;
    on_dragenter_capture, "onDragEnterCapture", DragEvent;
    on_dragexit, "onDragExit", DragEvent;
    on_dragexit_capture, "onDragExitCapture", DragEvent;
    on_dragleave, "onDragLeave", DragEvent;
    on_dragleave_capture, "onDragLeaveCapture", DragEvent;
    on_dragover, "onDragOver", DragEvent;
    on_dragover_capture, "onDragOverCapture", DragEvent;
    on_dragstart, "onDragStart", DragEvent;
    on_dragstart_capture, "onDragStartCapture", DragEvent;
    on_drop, "onDrop", DragEvent;
    on_drop_capture, "onDropCapture", DragEvent;

    on_scroll, "onScroll", UiEvent;
    on_scroll_capture, "onScrollCapture", UiEvent;
    on_wheel, "onWheel", WheelEvent;
    on_wheel_capture, "onWheelCapture", WheelEvent;

    on_animationstart, "onAnimationStart", AnimationEvent;
    on_animationstart_capture, "onAnimationStartCapture", AnimationEvent;
    on_animationend, "onAnimationEnd", AnimationEvent;
    on_animationend_capture, "onAnimationEndCapture", AnimationEvent;
    on_animationiteration, "onAnimationIteration", AnimationEvent;
    on_animationiteration_capture, "onAnimationIterationCapture", AnimationEvent;
    on_transition_end, "onTransitionEnd", TransitionEvent;
    on_transition_end_capture, "onTransitionEndCapture", TransitionEvent;
  }
}
