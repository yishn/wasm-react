use super::Attr;

macro_rules! impl_event {
  ($($on_event:ident, $on_event_str:expr, $evt:path);*;) => {
    $(
      pub fn $on_event(self, f: impl Fn($evt) + 'static) -> Self {
        self.insert_callback($on_event_str, f)
      }
    )*
  };
}

impl Attr {
  impl_event! {
    on_focus, "onFocus", web_sys::FocusEvent;
    on_focus_capture, "onFocusCapture", web_sys::FocusEvent;
    on_blur, "onBlur", web_sys::FocusEvent;
    on_blur_capture, "onBlurCapture", web_sys::FocusEvent;

    on_change, "onChange", web_sys::Event;
    on_change_capture, "onChangeCapture", web_sys::Event;
    on_before_input, "onBeforeInput", web_sys::Event;
    on_before_input_capture, "onBeforeInputCapture", web_sys::Event;
    on_input, "onInput", web_sys::Event;
    on_input_capture, "onInputCapture", web_sys::Event;
    on_reset, "onReset", web_sys::Event;
    on_reset_capture, "onResetCapture", web_sys::Event;
    on_submit, "onSubmit", web_sys::Event;
    on_submit_capture, "onSubmitCapture", web_sys::Event;
    on_invalid, "onInvalid", web_sys::Event;
    on_invalid_capture, "onInvalidCapture", web_sys::Event;
    on_select, "onSelect", web_sys::UiEvent;
    on_select_capture, "onSelectCapture", web_sys::UiEvent;

    on_load, "onLoad", web_sys::Event;
    on_load_capture, "onLoadCapture", web_sys::Event;

    on_key_down, "onKeyDown", web_sys::KeyboardEvent;
    on_key_down_capture, "onKeyDownCapture", web_sys::KeyboardEvent;
    on_key_press, "onKeyPress", web_sys::KeyboardEvent;
    on_key_press_capture, "onKeyPressCapture", web_sys::KeyboardEvent;
    on_key_up, "onKeyUp", web_sys::KeyboardEvent;
    on_key_up_capture, "onKeyUpCapture", web_sys::KeyboardEvent;

    on_aux_click, "onAuxClick", web_sys::MouseEvent;
    on_aux_click_capture, "onAuxClickCapture", web_sys::MouseEvent;
    on_click, "onClick", web_sys::MouseEvent;
    on_click_capture, "onClickCapture", web_sys::MouseEvent;
    on_context_menu, "onContextMenu", web_sys::MouseEvent;
    on_context_menu_capture, "onContextMenuCapture", web_sys::MouseEvent;
    on_double_click, "onDoubleClick", web_sys::MouseEvent;
    on_double_click_capture, "onDoubleClickCapture", web_sys::MouseEvent;
    on_mouse_down, "onMouseDown", web_sys::MouseEvent;
    on_mouse_down_capture, "onMouseDownCapture", web_sys::MouseEvent;
    on_mouse_enter, "onMouseEnter", web_sys::MouseEvent;
    on_mouse_leave, "onMouseLeave", web_sys::MouseEvent;
    on_mouse_move, "onMouseMove", web_sys::MouseEvent;
    on_mouse_move_capture, "onMouseMoveCapture", web_sys::MouseEvent;
    on_mouse_out, "onMouseOut", web_sys::MouseEvent;
    on_mouse_out_capture, "onMouseOutCapture", web_sys::MouseEvent;
    on_mouse_over, "onMouseOver", web_sys::MouseEvent;
    on_mouse_over_capture, "onMouseOverCapture", web_sys::MouseEvent;
    on_mouse_up, "onMouseUp", web_sys::MouseEvent;
    on_mouse_up_capture, "onMouseUpCapture", web_sys::MouseEvent;

    on_pointer_down, "onPointerDown", web_sys::PointerEvent;
    on_pointer_down_capture, "onPointerDownCapture", web_sys::PointerEvent;
    on_pointer_move, "onPointerMove", web_sys::PointerEvent;
    on_pointer_move_capture, "onPointerMoveCapture", web_sys::PointerEvent;
    on_pointer_up, "onPointerUp", web_sys::PointerEvent;
    on_pointer_up_capture, "onPointerUpCapture", web_sys::PointerEvent;
    on_pointer_cancel, "onPointerCancel", web_sys::PointerEvent;
    on_pointer_cancel_capture, "onPointerCancelCapture", web_sys::PointerEvent;
    on_pointer_enter, "onPointerEnter", web_sys::PointerEvent;
    on_pointer_enter_capture, "onPointerEnterCapture", web_sys::PointerEvent;
    on_pointer_leave, "onPointerLeave", web_sys::PointerEvent;
    on_pointer_leave_capture, "onPointerLeaveCapture", web_sys::PointerEvent;
    on_pointer_over, "onPointerOver", web_sys::PointerEvent;
    on_pointer_over_capture, "onPointerOverCapture", web_sys::PointerEvent;
    on_pointer_out, "onPointerOut", web_sys::PointerEvent;
    on_pointer_out_capture, "onPointerOutCapture", web_sys::PointerEvent;
    on_got_pointer_capture, "onGotPointerCapture", web_sys::PointerEvent;
    on_got_pointer_capture_capture, "onGotPointerCaptureCapture", web_sys::PointerEvent;
    on_lost_pointer_capture, "onLostPointerCapture", web_sys::PointerEvent;
    on_lost_pointer_capture_capture, "onLostPointerCaptureCapture", web_sys::PointerEvent;

    on_drag, "onDrag", web_sys::DragEvent;
    on_drag_capture, "onDragCapture", web_sys::DragEvent;
    on_drag_end, "onDragEnd", web_sys::DragEvent;
    on_drag_end_capture, "onDragEndCapture", web_sys::DragEvent;
    on_drag_enter, "onDragEnter", web_sys::DragEvent;
    on_drag_enter_capture, "onDragEnterCapture", web_sys::DragEvent;
    on_drag_exit, "onDragExit", web_sys::DragEvent;
    on_drag_exit_capture, "onDragExitCapture", web_sys::DragEvent;
    on_drag_leave, "onDragLeave", web_sys::DragEvent;
    on_drag_leave_capture, "onDragLeaveCapture", web_sys::DragEvent;
    on_drag_over, "onDragOver", web_sys::DragEvent;
    on_drag_over_capture, "onDragOverCapture", web_sys::DragEvent;
    on_drag_start, "onDragStart", web_sys::DragEvent;
    on_drag_start_capture, "onDragStartCapture", web_sys::DragEvent;
    on_drop, "onDrop", web_sys::DragEvent;
    on_drop_capture, "onDropCapture", web_sys::DragEvent;

    on_scroll, "onScroll", web_sys::UiEvent;
    on_scroll_capture, "onScrollCapture", web_sys::UiEvent;
    on_wheel, "onWheel", web_sys::WheelEvent;
    on_wheel_capture, "onWheelCapture", web_sys::WheelEvent;

    on_animation_start, "onAnimationStart", web_sys::AnimationEvent;
    on_animation_start_capture, "onAnimationStartCapture", web_sys::AnimationEvent;
    on_animation_end, "onAnimationEnd", web_sys::AnimationEvent;
    on_animation_end_capture, "onAnimationEndCapture", web_sys::AnimationEvent;
    on_animation_iteration, "onAnimationIteration", web_sys::AnimationEvent;
    on_animation_iteration_capture, "onAnimationIterationCapture", web_sys::AnimationEvent;
    on_transition_end, "onTransitionEnd", web_sys::TransitionEvent;
    on_transition_end_capture, "onTransitionEndCapture", web_sys::TransitionEvent;
  }
}
