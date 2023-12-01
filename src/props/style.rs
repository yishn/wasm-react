use super::Props;
use wasm_bindgen::{intern, JsValue};

/// A convenience wrapper around [`Props`] that provides auto-completion for
/// style-related properties.
///
/// # Example
///
/// ```
/// # use wasm_react::props::*;
/// # fn f() -> Style {
/// Style::new()
///   .display("grid")
///   .grid("1fr 1fr / 1fr 1fr")
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Style(Props);

impl Style {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self(Props::new())
  }

  /// Equivalent to `style[key] = value;`.
  pub fn insert(self, key: &str, value: &JsValue) -> Self {
    Self(self.0.insert(key, value))
  }
}

impl AsRef<JsValue> for Style {
  fn as_ref(&self) -> &JsValue {
    self.0.as_ref()
  }
}

impl From<Style> for JsValue {
  fn from(style: Style) -> Self {
    style.0.into()
  }
}

macro_rules! impl_style {
  { $( $attr:ident, $attr_str:literal; )* } => {
    $(
      #[allow(missing_docs)]
      pub fn $attr(self, value: impl Into<JsValue>) -> Self {
        self.insert(intern($attr_str), &value.into())
      }
    )*
  };
}

impl Style {
  impl_style! {
    accent_color, "accentColor";
    align_content, "alignContent";
    align_items, "alignItems";
    align_self, "alignSelf";
    align_tracks, "alignTracks";
    all, "all";
    animation_delay, "animationDelay";
    animation_direction, "animationDirection";
    animation_duration, "animationDuration";
    animation_fill_mode, "animationFillMode";
    animation_iteration_count, "animationIterationCount";
    animation_name, "animationName";
    animation_play_state, "animationPlayState";
    animation_timeline, "animationTimeline";
    animation_timing_function, "animationTimingFunction";
    animation, "animation";
    appearance, "appearance";
    aspect_ratio, "aspectRatio";
    backdrop_filter, "backdropFilter";
    backface_visibility, "backfaceVisibility";
    background_attachment, "backgroundAttachment";
    background_blend_mode, "backgroundBlendMode";
    background_clip, "backgroundClip";
    background_color, "backgroundColor";
    background_image, "backgroundImage";
    background_origin, "backgroundOrigin";
    background_position_x, "backgroundPositionX";
    background_position_y, "backgroundPositionY";
    background_position, "backgroundPosition";
    background_repeat, "backgroundRepeat";
    background_size, "backgroundSize";
    background, "background";
    block_overflow, "blockOverflow";
    block_size, "blockSize";
    border_block_color, "borderBlockColor";
    border_block_end_color, "borderBlockEndColor";
    border_block_end_style, "borderBlockEndStyle";
    border_block_end_width, "borderBlockEndWidth";
    border_block_end, "borderBlockEnd";
    border_block_start_color, "borderBlockStartColor";
    border_block_start_style, "borderBlockStartStyle";
    border_block_start_width, "borderBlockStartWidth";
    border_block_start, "borderBlockStart";
    border_block_style, "borderBlockStyle";
    border_block_width, "borderBlockWidth";
    border_block, "borderBlock";
    border_bottom_color, "borderBottomColor";
    border_bottom_left_radius, "borderBottomLeftRadius";
    border_bottom_right_radius, "borderBottomRightRadius";
    border_bottom_style, "borderBottomStyle";
    border_bottom_width, "borderBottomWidth";
    border_bottom, "borderBottom";
    border_collapse, "borderCollapse";
    border_color, "borderColor";
    border_end_end_radius, "borderEndEndRadius";
    border_end_start_radius, "borderEndStartRadius";
    border_image_outset, "borderImageOutset";
    border_image_repeat, "borderImageRepeat";
    border_image_slice, "borderImageSlice";
    border_image_source, "borderImageSource";
    border_image_width, "borderImageWidth";
    border_image, "borderImage";
    border_inline_color, "borderInlineColor";
    border_inline_end_color, "borderInlineEndColor";
    border_inline_end_style, "borderInlineEndStyle";
    border_inline_end_width, "borderInlineEndWidth";
    border_inline_end, "borderInlineEnd";
    border_inline_start_color, "borderInlineStartColor";
    border_inline_start_style, "borderInlineStartStyle";
    border_inline_start_width, "borderInlineStartWidth";
    border_inline_start, "borderInlineStart";
    border_inline_style, "borderInlineStyle";
    border_inline_width, "borderInlineWidth";
    border_inline, "borderInline";
    border_left_color, "borderLeftColor";
    border_left_style, "borderLeftStyle";
    border_left_width, "borderLeftWidth";
    border_left, "borderLeft";
    border_radius, "borderRadius";
    border_right_color, "borderRightColor";
    border_right_style, "borderRightStyle";
    border_right_width, "borderRightWidth";
    border_right, "borderRight";
    border_spacing, "borderSpacing";
    border_start_end_radius, "borderStartEndRadius";
    border_start_start_radius, "borderStartStartRadius";
    border_style, "borderStyle";
    border_top_color, "borderTopColor";
    border_top_left_radius, "borderTopLeftRadius";
    border_top_right_radius, "borderTopRightRadius";
    border_top_style, "borderTopStyle";
    border_top_width, "borderTopWidth";
    border_top, "borderTop";
    border_width, "borderWidth";
    border, "border";
    bottom, "bottom";
    box_decoration_break, "boxDecorationBreak";
    box_shadow, "boxShadow";
    box_sizing, "boxSizing";
    break_after, "breakAfter";
    break_before, "breakBefore";
    break_inside, "breakInside";
    caption_side, "captionSide";
    caret_color, "caretColor";
    clear, "clear";
    clip_path, "clipPath";
    color_adjust, "colorAdjust";
    color_scheme, "colorScheme";
    color, "color";
    column_count, "columnCount";
    column_fill, "columnFill";
    column_gap, "columnGap";
    column_rule_color, "columnRuleColor";
    column_rule_style, "columnRuleStyle";
    column_rule_width, "columnRuleWidth";
    column_rule, "columnRule";
    column_span, "columnSpan";
    column_width, "columnWidth";
    columns, "columns";
    contain, "contain";
    content_visibility, "contentVisibility";
    content, "content";
    counter_increment, "counterIncrement";
    counter_reset, "counterReset";
    counter_set, "counterSet";
    cursor, "cursor";
    direction, "direction";
    display, "display";
    empty_cells, "emptyCells";
    filter, "filter";
    flex_basis, "flexBasis";
    flex_direction, "flexDirection";
    flex_flow, "flexFlow";
    flex_grow, "flexGrow";
    flex_shrink, "flexShrink";
    flex_wrap, "flexWrap";
    flex, "flex";
    float, "float";
    font_family, "fontFamily";
    font_feature_settings, "fontFeatureSettings";
    font_kerning, "fontKerning";
    font_language_override, "fontLanguageOverride";
    font_optical_sizing, "fontOpticalSizing";
    font_size_adjust, "fontSizeAdjust";
    font_size, "fontSize";
    font_smooth, "fontSmooth";
    font_stretch, "fontStretch";
    font_style, "fontStyle";
    font_synthesis, "fontSynthesis";
    font_variant_alternates, "fontVariantAlternates";
    font_variant_caps, "fontVariantCaps";
    font_variant_east_asian, "fontVariantEastAsian";
    font_variant_ligatures, "fontVariantLigatures";
    font_variant_numeric, "fontVariantNumeric";
    font_variant_position, "fontVariantPosition";
    font_variant, "fontVariant";
    font_variation_settings, "fontVariationSettings";
    font_weight, "fontWeight";
    font, "font";
    forced_color_adjust, "forcedColorAdjust";
    gap, "gap";
    grid_area, "gridArea";
    grid_auto_columns, "gridAutoColumns";
    grid_auto_flow, "gridAutoFlow";
    grid_auto_rows, "gridAutoRows";
    grid_column_end, "gridColumnEnd";
    grid_column_start, "gridColumnStart";
    grid_column, "gridColumn";
    grid_row_end, "gridRowEnd";
    grid_row_start, "gridRowStart";
    grid_row, "gridRow";
    grid_template_areas, "gridTemplateAreas";
    grid_template_columns, "gridTemplateColumns";
    grid_template_rows, "gridTemplateRows";
    grid_template, "gridTemplate";
    grid, "grid";
    hanging_punctuation, "hangingPunctuation";
    height, "height";
    hyphenate_character, "hyphenateCharacter";
    hyphens, "hyphens";
    image_orientation, "imageOrientation";
    image_rendering, "imageRendering";
    image_resolution, "imageResolution";
    initial_letter, "initialLetter";
    inline_size, "inlineSize";
    input_security, "inputSecurity";
    inset_block_end, "insetBlockEnd";
    inset_block_start, "insetBlockStart";
    inset_block, "insetBlock";
    inset_inline_end, "insetInlineEnd";
    inset_inline_start, "insetInlineStart";
    inset_inline, "insetInline";
    inset, "inset";
    isolation, "isolation";
    justify_content, "justifyContent";
    justify_items, "justifyItems";
    justify_self, "justifySelf";
    justify_tracks, "justifyTracks";
    left, "left";
    letter_spacing, "letterSpacing";
    line_break, "lineBreak";
    line_clamp, "lineClamp";
    line_height_step, "lineHeightStep";
    line_height, "lineHeight";
    list_style_image, "listStyleImage";
    list_style_position, "listStylePosition";
    list_style_type, "listStyleType";
    list_style, "listStyle";
    margin_block_end, "marginBlockEnd";
    margin_block_start, "marginBlockStart";
    margin_block, "marginBlock";
    margin_bottom, "marginBottom";
    margin_inline_end, "marginInlineEnd";
    margin_inline_start, "marginInlineStart";
    margin_inline, "marginInline";
    margin_left, "marginLeft";
    margin_right, "marginRight";
    margin_top, "marginTop";
    margin, "margin";
    mask_border_mode, "maskBorderMode";
    mask_border_outset, "maskBorderOutset";
    mask_border_repeat, "maskBorderRepeat";
    mask_border_slice, "maskBorderSlice";
    mask_border_source, "maskBorderSource";
    mask_border_width, "maskBorderWidth";
    mask_border, "maskBorder";
    mask_clip, "maskClip";
    mask_composite, "maskComposite";
    mask_image, "maskImage";
    mask_mode, "maskMode";
    mask_origin, "maskOrigin";
    mask_position, "maskPosition";
    mask_repeat, "maskRepeat";
    mask_size, "maskSize";
    mask_type, "maskType";
    mask, "mask";
    math_style, "mathStyle";
    max_block_size, "maxBlockSize";
    max_height, "maxHeight";
    max_inline_size, "maxInlineSize";
    max_lines, "maxLines";
    max_width, "maxWidth";
    min_block_size, "minBlockSize";
    min_height, "minHeight";
    min_inline_size, "minInlineSize";
    min_width, "minWidth";
    mix_blend_mode, "mixBlendMode";
    motion_distance, "motionDistance";
    motion_path, "motionPath";
    motion_rotation, "motionRotation";
    motion, "motion";
    object_fit, "objectFit";
    object_position, "objectPosition";
    offset_anchor, "offsetAnchor";
    offset_distance, "offsetDistance";
    offset_path, "offsetPath";
    offset_rotate, "offsetRotate";
    offset_rotation, "offsetRotation";
    offset, "offset";
    opacity, "opacity";
    order, "order";
    orphans, "orphans";
    outline_color, "outlineColor";
    outline_offset, "outlineOffset";
    outline_style, "outlineStyle";
    outline_width, "outlineWidth";
    outline, "outline";
    overflow_anchor, "overflowAnchor";
    overflow_block, "overflowBlock";
    overflow_clip_box, "overflowClipBox";
    overflow_clip_margin, "overflowClipMargin";
    overflow_inline, "overflowInline";
    overflow_wrap, "overflowWrap";
    overflow_x, "overflowX";
    overflow_y, "overflowY";
    overflow, "overflow";
    overscroll_behavior_block, "overscrollBehaviorBlock";
    overscroll_behavior_inline, "overscrollBehaviorInline";
    overscroll_behavior_x, "overscrollBehaviorX";
    overscroll_behavior_y, "overscrollBehaviorY";
    overscroll_behavior, "overscrollBehavior";
    padding_block_end, "paddingBlockEnd";
    padding_block_start, "paddingBlockStart";
    padding_block, "paddingBlock";
    padding_bottom, "paddingBottom";
    padding_inline_end, "paddingInlineEnd";
    padding_inline_start, "paddingInlineStart";
    padding_inline, "paddingInline";
    padding_left, "paddingLeft";
    padding_right, "paddingRight";
    padding_top, "paddingTop";
    padding, "padding";
    page_break_after, "pageBreakAfter";
    page_break_before, "pageBreakBefore";
    page_break_inside, "pageBreakInside";
    paint_order, "paintOrder";
    perspective_origin, "perspectiveOrigin";
    perspective, "perspective";
    place_content, "placeContent";
    place_items, "placeItems";
    place_self, "placeSelf";
    pointer_events, "pointerEvents";
    position, "position";
    print_color_adjust, "printColorAdjust";
    quotes, "quotes";
    resize, "resize";
    right, "right";
    rotate, "rotate";
    row_gap, "rowGap";
    ruby_align, "rubyAlign";
    ruby_merge, "rubyMerge";
    ruby_position, "rubyPosition";
    scale, "scale";
    scroll_behavior, "scrollBehavior";
    scroll_margin_block_end, "scrollMarginBlockEnd";
    scroll_margin_block_start, "scrollMarginBlockStart";
    scroll_margin_block, "scrollMarginBlock";
    scroll_margin_bottom, "scrollMarginBottom";
    scroll_margin_inline_end, "scrollMarginInlineEnd";
    scroll_margin_inline_start, "scrollMarginInlineStart";
    scroll_margin_inline, "scrollMarginInline";
    scroll_margin_left, "scrollMarginLeft";
    scroll_margin_right, "scrollMarginRight";
    scroll_margin_top, "scrollMarginTop";
    scroll_margin, "scrollMargin";
    scroll_padding_block_end, "scrollPaddingBlockEnd";
    scroll_padding_block_start, "scrollPaddingBlockStart";
    scroll_padding_block, "scrollPaddingBlock";
    scroll_padding_bottom, "scrollPaddingBottom";
    scroll_padding_inline_end, "scrollPaddingInlineEnd";
    scroll_padding_inline_start, "scrollPaddingInlineStart";
    scroll_padding_inline, "scrollPaddingInline";
    scroll_padding_left, "scrollPaddingLeft";
    scroll_padding_right, "scrollPaddingRight";
    scroll_padding_top, "scrollPaddingTop";
    scroll_padding, "scrollPadding";
    scroll_snap_align, "scrollSnapAlign";
    scroll_snap_margin_bottom, "scrollSnapMarginBottom";
    scroll_snap_margin_left, "scrollSnapMarginLeft";
    scroll_snap_margin_right, "scrollSnapMarginRight";
    scroll_snap_margin_top, "scrollSnapMarginTop";
    scroll_snap_margin, "scrollSnapMargin";
    scroll_snap_stop, "scrollSnapStop";
    scroll_snap_type, "scrollSnapType";
    scrollbar_color, "scrollbarColor";
    scrollbar_gutter, "scrollbarGutter";
    scrollbar_width, "scrollbarWidth";
    shape_image_threshold, "shapeImageThreshold";
    shape_margin, "shapeMargin";
    shape_outside, "shapeOutside";
    tab_size, "tabSize";
    table_layout, "tableLayout";
    text_align_last, "textAlignLast";
    text_align, "textAlign";
    text_combine_upright, "textCombineUpright";
    text_decoration_color, "textDecorationColor";
    text_decoration_line, "textDecorationLine";
    text_decoration_skip_ink, "textDecorationSkipInk";
    text_decoration_skip, "textDecorationSkip";
    text_decoration_style, "textDecorationStyle";
    text_decoration_thickness, "textDecorationThickness";
    text_decoration_width, "textDecorationWidth";
    text_decoration, "textDecoration";
    text_emphasis_color, "textEmphasisColor";
    text_emphasis_position, "textEmphasisPosition";
    text_emphasis_style, "textEmphasisStyle";
    text_emphasis, "textEmphasis";
    text_indent, "textIndent";
    text_justify, "textJustify";
    text_orientation, "textOrientation";
    text_overflow, "textOverflow";
    text_rendering, "textRendering";
    text_shadow, "textShadow";
    text_size_adjust, "textSizeAdjust";
    text_transform, "textTransform";
    text_underline_offset, "textUnderlineOffset";
    text_underline_position, "textUnderlinePosition";
    top, "top";
    touch_action, "touchAction";
    transform_box, "transformBox";
    transform_origin, "transformOrigin";
    transform_style, "transformStyle";
    transform, "transform";
    transition_delay, "transitionDelay";
    transition_duration, "transitionDuration";
    transition_property, "transitionProperty";
    transition_timing_function, "transitionTimingFunction";
    transition, "transition";
    translate, "translate";
    unicode_bidi, "unicodeBidi";
    user_select, "userSelect";
    vertical_align, "verticalAlign";
    visibility, "visibility";
    white_space, "whiteSpace";
    widows, "widows";
    width, "width";
    will_change, "willChange";
    word_break, "wordBreak";
    word_spacing, "wordSpacing";
    word_wrap, "wordWrap";
    writing_mode, "writingMode";
    z_index, "zIndex";
    zoom, "zoom";

    // SVG styles
    alignment_baseline, "alignmentBaseline";
    baseline_shift, "baselineShift";
    clip, "clip";
    clip_rule, "clipRule";
    color_interpolation, "colorInterpolation";
    color_rendering, "colorRendering";
    dominant_baseline, "dominantBaseline";
    fill, "fill";
    fill_opacity, "fillOpacity";
    fill_rule, "fillRule";
    flood_color, "floodColor";
    flood_opacity, "floodOpacity";
    glyph_orientation_vertical, "glyphOrientationVertical";
    lighting_color, "lightingColor";
    marker, "marker";
    marker_end, "markerEnd";
    marker_mid, "markerMid";
    marker_start, "markerStart";
    shape_rendering, "shapeRendering";
    stop_color, "stopColor";
    stop_opacity, "stopOpacity";
    stroke, "stroke";
    stroke_dasharray, "strokeDasharray";
    stroke_dashoffset, "strokeDashoffset";
    stroke_linecap, "strokeLinecap";
    stroke_linejoin, "strokeLinejoin";
    stroke_miterlimit, "strokeMiterlimit";
    stroke_opacity, "strokeOpacity";
    stroke_width, "strokeWidth";
    text_anchor, "textAnchor";
    vector_effect, "vectorEffect";
  }
}
