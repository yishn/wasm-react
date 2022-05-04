use super::Props;
use crate::Callback;
use wasm_bindgen::{
  convert::{FromWasmAbi, IntoWasmAbi},
  JsValue,
};

/// To be used with [`Attr::dangerously_set_inner_html()`].
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DangerousHtml {
  pub __html: String,
}

/// A convenience wrapper around [`Props`] that provides auto-completion for
/// HTML attributes.
#[derive(Debug, Default, Clone)]
pub struct Attr(Props);

impl Attr {
  /// Creates a new, empty object.
  pub fn new() -> Self {
    Self(Props::new())
  }

  /// Equivalent to `props[key] = value;`.
  pub fn insert(self, key: &str, value: impl Into<JsValue>) -> Self {
    Self(self.0.insert(key, value.into()))
  }

  /// Equivalent to `props[key] = f;`.
  pub fn insert_callback<T, U>(
    self,
    key: &str,
    f: impl Fn(T) -> U + 'static,
  ) -> Self
  where
    T: FromWasmAbi + 'static,
    U: IntoWasmAbi + 'static,
  {
    Self(self.0.insert(key, Callback::new(f)))
  }

  /// Equivalent to `props.dangerouslySetInnerHTML = { __html: value.__html };`.
  ///
  /// See also [React documentation](https://reactjs.org/docs/dom-elements.html#dangerouslysetinnerhtml).
  ///
  /// # Example
  ///
  /// ```
  /// fn create_markup() -> DangerousHtml<'static> {
  ///   DangerousHtml {
  ///     __html: "First &middot; Second".into()
  ///   }
  /// }
  ///
  /// html("div", Attr::new().dangerously_set_inner_html(create_markup()), [])
  /// ```
  pub fn dangerously_set_inner_html(self, value: DangerousHtml) -> Self {
    Self(self.0.insert(
      "dangerouslySetInnerHTML",
      Props::new().insert("__html", value.__html),
    ))
  }
}

impl From<Attr> for JsValue {
  fn from(attr: Attr) -> Self {
    attr.0.into()
  }
}

macro_rules! impl_attr {
  ($($attr:ident, $attr_str:expr, $value:ty);*;) => {
    $(
      pub fn $attr(self, value: $value) -> Self {
        self.insert($attr_str, value)
      }
    )*
  };
}

impl Attr {
  impl_attr! {
    // Standard HTML Attributes
    access_key, "accessKey", &str;
    content_editable, "contentEditable", bool;
    context_menu, "contextMenu", &str;
    dir, "dir", &str;
    draggable, "draggable", bool;
    hidden, "hidden", bool;
    id, "id", &str;
    lang, "lang", &str;
    placeholder, "placeholder", &str;
    slot, "slot", &str;
    spell_check, "spellCheck", bool;
    tab_index, "tabIndex", i32;
    title, "title", &str;
    translate, "translate", &str;
    radio_group, "radioGroup", &str;

    // WAI-ARIA
    role, "role", &str;

    // RDFa Attributes
    about, "about", &str;
    datatype, "datatype", &str;
    inlist, "inlist", impl Into<JsValue>;
    prefix, "prefix", &str;
    property, "property", &str;
    resource, "resource", &str;
    vocab, "vocab", &str;

    // Living Standard
    input_mode, "inputMode", &str;
    is, "is", &str;

    // Standard HTML Attributes
    accept, "accept", &str;
    accept_charset, "acceptCharset", &str;
    action, "action", &str;
    allow_full_screen, "allowFullScreen", bool;
    allow_transparency, "allowTransparency", bool;
    alt, "alt", &str;
    auto_complete, "autoComplete", &str;
    auto_focus, "autoFocus", bool;
    auto_play, "autoPlay", bool;
    capture, "capture", impl Into<JsValue>;
    cell_padding, "cellPadding", impl Into<JsValue>;
    cell_spacing, "cellSpacing", impl Into<JsValue>;
    char_set, "charSet", &str;
    challenge, "challenge", &str;
    checked, "checked", bool;
    cite, "cite", &str;
    class_id, "classID", &str;
    cols, "cols", u32;
    col_span, "colSpan", u32;
    content, "content", &str;
    controls, "controls", bool;
    coords, "coords", &str;
    cross_origin, "crossOrigin", &str;
    data, "data", &str;
    date_time, "dateTime", &str;
    default, "default", bool;
    defer, "defer", bool;
    disabled, "disabled", bool;
    download, "download", impl Into<JsValue>;
    enc_type, "encType", &str;
    form, "form", &str;
    form_action, "formAction", &str;
    form_enc_type, "formEncType", &str;
    form_method, "formMethod", &str;
    form_no_validate, "formNoValidate", bool;
    form_target, "formTarget", &str;
    frame_border, "frameBorder", impl Into<JsValue>;
    headers, "headers", &str;
    height, "height", impl Into<JsValue>;
    high, "high", f64;
    href, "href", &str;
    href_lang, "hrefLang", &str;
    html_for, "htmlFor", &str;
    http_equiv, "httpEquiv", &str;
    integrity, "integrity", &str;
    key_params, "keyParams", &str;
    key_type, "keyType", &str;
    kind, "kind", &str;
    label, "label", &str;
    list, "list", &str;
    low, "low", f64;
    manifest, "manifest", &str;
    margin_height, "marginHeight", f64;
    margin_width, "marginWidth", f64;
    max, "max", f64;
    max_length, "maxLength", f64;
    media, "media", &str;
    media_group, "mediaGroup", &str;
    method, "method", &str;
    min, "min", impl Into<JsValue>;
    min_length, "minLength", f64;
    multiple, "multiple", bool;
    muted, "muted", bool;
    name, "name", &str;
    nonce, "nonce", &str;
    no_validate, "noValidate", bool;
    open, "open", bool;
    optimum, "optimum", f64;
    pattern, "pattern", &str;
    plays_inline, "playsInline", bool;
    poster, "poster", &str;
    preload, "preload", &str;
    read_only, "readOnly", bool;
    rel, "rel", &str;
    required, "required", bool;
    reversed, "reversed", bool;
    rows, "rows", u32;
    row_span, "rowSpan", u32;
    sandbox, "sandbox", &str;
    scope, "scope", &str;
    scoped, "scoped", bool;
    scrolling, "scrolling", &str;
    seamless, "seamless", bool;
    selected, "selected", bool;
    shape, "shape", &str;
    size, "size", f64;
    sizes, "sizes", &str;
    span, "span", u32;
    src, "src", &str;
    src_doc, "srcDoc", &str;
    src_lang, "srcLang", &str;
    src_set, "srcSet", &str;
    start, "start", f64;
    step, "step", impl Into<JsValue>;
    summary, "summary", &str;
    target, "target", &str;
    typ, "type", &str;
    use_map, "useMap", &str;
    value, "value", impl Into<JsValue>;
    width, "width", impl Into<JsValue>;
    wmode, "wmode", &str;
    wrap, "wrap", &str;
  }
}
