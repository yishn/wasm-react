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
    on_before_input, "onBeforeInput", Event;
    on_before_input_capture, "onBeforeInputCapture", Event;
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

    on_key_down, "onKeyDown", KeyboardEvent;
    on_key_down_capture, "onKeyDownCapture", KeyboardEvent;
    on_key_press, "onKeyPress", KeyboardEvent;
    on_key_press_capture, "onKeyPressCapture", KeyboardEvent;
    on_key_up, "onKeyUp", KeyboardEvent;
    on_key_up_capture, "onKeyUpCapture", KeyboardEvent;

    on_abort, "onAbort", Event;
    on_abort_capture, "onAbortCapture", Event;
    on_can_play, "onCanPlay", Event;
    on_can_play_capture, "onCanPlayCapture", Event;
    on_can_play_through, "onCanPlayThrough", Event;
    on_can_play_through_capture, "onCanPlayThroughCapture", Event;
    on_duration_change, "onDurationChange", Event;
    on_duration_change_capture, "onDurationChangeCapture", Event;
    on_emptied, "onEmptied", Event;
    on_emptied_capture, "onEmptiedCapture", Event;
    on_encrypted, "onEncrypted", Event;
    on_encrypted_capture, "onEncryptedCapture", Event;
    on_ended, "onEnded", Event;
    on_ended_capture, "onEndedCapture", Event;
    on_loaded_data, "onLoadedData", Event;
    on_loaded_data_capture, "onLoadedDataCapture", Event;
    on_loaded_metadata, "onLoadedMetadata", Event;
    on_loaded_metadata_capture, "onLoadedMetadataCapture", Event;
    on_load_start, "onLoadStart", Event;
    on_load_start_capture, "onLoadStartCapture", Event;
    on_pause, "onPause", Event;
    on_pause_capture, "onPauseCapture", Event;
    on_play, "onPlay", Event;
    on_play_capture, "onPlayCapture", Event;
    on_playing, "onPlaying", Event;
    on_playing_capture, "onPlayingCapture", Event;
    on_progress, "onProgress", Event;
    on_progress_capture, "onProgressCapture", Event;
    on_rate_change, "onRateChange", Event;
    on_rate_change_capture, "onRateChangeCapture", Event;
    on_seeked, "onSeeked", Event;
    on_seeked_capture, "onSeekedCapture", Event;
    on_seeking, "onSeeking", Event;
    on_seeking_capture, "onSeekingCapture", Event;
    on_stalled, "onStalled", Event;
    on_stalled_capture, "onStalledCapture", Event;
    on_suspend, "onSuspend", Event;
    on_suspend_capture, "onSuspendCapture", Event;
    on_time_update, "onTimeUpdate", Event;
    on_time_update_capture, "onTimeUpdateCapture", Event;
    on_volume_change, "onVolumeChange", Event;
    on_volume_change_capture, "onVolumeChangeCapture", Event;
    on_waiting, "onWaiting", Event;
    on_waiting_capture, "onWaitingCapture", Event;

    on_aux_click, "onAuxClick", MouseEvent;
    on_aux_click_capture, "onAuxClickCapture", MouseEvent;
    on_click, "onClick", MouseEvent;
    on_click_capture, "onClickCapture", MouseEvent;
    on_context_menu, "onContextMenu", MouseEvent;
    on_context_menu_capture, "onContextMenuCapture", MouseEvent;
    on_double_click, "onDoubleClick", MouseEvent;
    on_double_click_capture, "onDoubleClickCapture", MouseEvent;
    on_mouse_down, "onMouseDown", MouseEvent;
    on_mouse_down_capture, "onMouseDownCapture", MouseEvent;
    on_mouse_enter, "onMouseEnter", MouseEvent;
    on_mouse_leave, "onMouseLeave", MouseEvent;
    on_mouse_move, "onMouseMove", MouseEvent;
    on_mouse_move_capture, "onMouseMoveCapture", MouseEvent;
    on_mouse_out, "onMouseOut", MouseEvent;
    on_mouse_out_capture, "onMouseOutCapture", MouseEvent;
    on_mouse_over, "onMouseOver", MouseEvent;
    on_mouse_over_capture, "onMouseOverCapture", MouseEvent;
    on_mouse_up, "onMouseUp", MouseEvent;
    on_mouse_up_capture, "onMouseUpCapture", MouseEvent;

    on_pointer_down, "onPointerDown", PointerEvent;
    on_pointer_down_capture, "onPointerDownCapture", PointerEvent;
    on_pointer_move, "onPointerMove", PointerEvent;
    on_pointer_move_capture, "onPointerMoveCapture", PointerEvent;
    on_pointer_up, "onPointerUp", PointerEvent;
    on_pointer_up_capture, "onPointerUpCapture", PointerEvent;
    on_pointer_cancel, "onPointerCancel", PointerEvent;
    on_pointer_cancel_capture, "onPointerCancelCapture", PointerEvent;
    on_pointer_enter, "onPointerEnter", PointerEvent;
    on_pointer_enter_capture, "onPointerEnterCapture", PointerEvent;
    on_pointer_leave, "onPointerLeave", PointerEvent;
    on_pointer_leave_capture, "onPointerLeaveCapture", PointerEvent;
    on_pointer_over, "onPointerOver", PointerEvent;
    on_pointer_over_capture, "onPointerOverCapture", PointerEvent;
    on_pointer_out, "onPointerOut", PointerEvent;
    on_pointer_out_capture, "onPointerOutCapture", PointerEvent;
    on_got_pointer_capture, "onGotPointerCapture", PointerEvent;
    on_got_pointer_capture_capture, "onGotPointerCaptureCapture", PointerEvent;
    on_lost_pointer_capture, "onLostPointerCapture", PointerEvent;
    on_lost_pointer_capture_capture, "onLostPointerCaptureCapture", PointerEvent;

    on_drag, "onDrag", DragEvent;
    on_drag_capture, "onDragCapture", DragEvent;
    on_drag_end, "onDragEnd", DragEvent;
    on_drag_end_capture, "onDragEndCapture", DragEvent;
    on_drag_enter, "onDragEnter", DragEvent;
    on_drag_enter_capture, "onDragEnterCapture", DragEvent;
    on_drag_exit, "onDragExit", DragEvent;
    on_drag_exit_capture, "onDragExitCapture", DragEvent;
    on_drag_leave, "onDragLeave", DragEvent;
    on_drag_leave_capture, "onDragLeaveCapture", DragEvent;
    on_drag_over, "onDragOver", DragEvent;
    on_drag_over_capture, "onDragOverCapture", DragEvent;
    on_drag_start, "onDragStart", DragEvent;
    on_drag_start_capture, "onDragStartCapture", DragEvent;
    on_drop, "onDrop", DragEvent;
    on_drop_capture, "onDropCapture", DragEvent;

    on_scroll, "onScroll", UiEvent;
    on_scroll_capture, "onScrollCapture", UiEvent;
    on_wheel, "onWheel", WheelEvent;
    on_wheel_capture, "onWheelCapture", WheelEvent;

    on_animation_start, "onAnimationStart", AnimationEvent;
    on_animation_start_capture, "onAnimationStartCapture", AnimationEvent;
    on_animation_end, "onAnimationEnd", AnimationEvent;
    on_animation_end_capture, "onAnimationEndCapture", AnimationEvent;
    on_animation_iteration, "onAnimationIteration", AnimationEvent;
    on_animation_iteration_capture, "onAnimationIterationCapture", AnimationEvent;
    on_transition_end, "onTransitionEnd", TransitionEvent;
    on_transition_end_capture, "onTransitionEndCapture", TransitionEvent;
  }
}
