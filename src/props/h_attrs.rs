use super::{HtmlTag, H};
use super::{Props, Style};
use std::borrow::Cow;
use wasm_bindgen::JsValue;

/// To be used with [`H::dangerously_set_inner_html()`].
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DangerousHtml<'a> {
  /// The HTML content to be rendered.
  pub __html: Cow<'a, str>,
}

macro_rules! impl_attr {
  { $( $attr:ident, $attr_str:literal => $T:ty; )* } => {
    $(
      #[allow(missing_docs)]
      pub fn $attr(self, value: $T) -> Self {
        self.attr($attr_str, &Into::into(value))
      }
    )*
  };
}

/// Provides auto-completion for DOM attributes on [`H`].
impl<'a> H<HtmlTag<'a>> {
  /// Equivalent to `props.dangerouslySetInnerHTML = { __html: value.__html };`.
  ///
  /// See also [React documentation](https://reactjs.org/docs/dom-elements.html#dangerouslysetinnerhtml).
  ///
  /// # Example
  ///
  /// ```
  /// # use wasm_react::{*, props::*};
  /// fn create_markup() -> DangerousHtml<'static> {
  ///   DangerousHtml {
  ///     __html: "First &middot; Second".into()
  ///   }
  /// }
  ///
  /// # fn f() -> VNode {
  /// h!(div)
  ///   .dangerously_set_inner_html(&create_markup())
  ///   .build(c![])
  /// # }
  /// ```
  pub fn dangerously_set_inner_html(self, value: &DangerousHtml) -> Self {
    self.attr(
      "dangerouslySetInnerHTML",
      Props::new()
        .insert("__html", &value.__html[..].into())
        .as_ref(),
    )
  }

  /// Overwrites the class name attribute. Use [`h!`](crate::h) for easier way
  /// to set the class names.
  pub fn class_name(self, value: &str) -> Self {
    self.attr("className", &value.into())
  }

  /// Sets the style attribute.
  pub fn style(self, style: &Style) -> Self {
    self.attr("style", style.as_ref())
  }

  impl_attr! {
    // Standard HTML Attributes
    accesskey, "accessKey" => &str;
    contenteditable, "contentEditable" => bool;
    contextmenu, "contextMenu" => &str;
    dir, "dir" => &str;
    draggable, "draggable" => bool;
    hidden, "hidden" => bool;
    id, "id" => &str;
    lang, "lang" => &str;
    placeholder, "placeholder" => &str;
    slot, "slot" => &str;
    spellcheck, "spellCheck" => bool;
    tabindex, "tabIndex" => i32;
    title, "title" => &str;
    translate, "translate" => &str;
    radiogroup, "radioGroup" => &str;

    // WAI-ARIA
    role, "role" => &str;

    // RDFa Attributes
    about, "about" => &str;
    datatype, "datatype" => &str;
    inlist, "inlist" => impl Into<JsValue>;
    prefix, "prefix" => &str;
    property, "property" => &str;
    resource, "resource" => &str;
    vocab, "vocab" => &str;

    // Living Standard
    inputmode, "inputMode" => &str;
    is, "is" => &str;

    // Standard HTML Attributes
    accept, "accept" => &str;
    acceptcharset, "acceptCharset" => &str;
    action, "action" => &str;
    allowfullscreen, "allowFullScreen" => bool;
    allowtransparency, "allowTransparency" => bool;
    alt, "alt" => &str;
    autocomplete, "autoComplete" => &str;
    autofocus, "autoFocus" => bool;
    autoplay, "autoPlay" => bool;
    capture, "capture" => impl Into<JsValue>;
    cellpadding, "cellPadding" => impl Into<JsValue>;
    cellspacing, "cellSpacing" => impl Into<JsValue>;
    challenge, "challenge" => &str;
    charset, "charSet" => &str;
    checked, "checked" => bool;
    cite, "cite" => &str;
    classid, "classID" => &str;
    cols, "cols" => u32;
    colspan, "colSpan" => u32;
    content, "content" => &str;
    controls, "controls" => bool;
    coords, "coords" => &str;
    crossorigin, "crossOrigin" => &str;
    data, "data" => &str;
    datetime, "dateTime" => &str;
    default, "default" => bool;
    defer, "defer" => bool;
    disabled, "disabled" => bool;
    download, "download" => impl Into<JsValue>;
    enctype, "encType" => &str;
    form, "form" => &str;
    formaction, "formAction" => &str;
    formenctype, "formEncType" => &str;
    formmethod, "formMethod" => &str;
    formnovalidate, "formNoValidate" => bool;
    formtarget, "formTarget" => &str;
    frameborder, "frameBorder" => impl Into<JsValue>;
    headers, "headers" => &str;
    height, "height" => impl Into<JsValue>;
    high, "high" => f64;
    href, "href" => &str;
    hreflang, "hrefLang" => &str;
    html_for, "htmlFor" => &str;
    html_type, "type" => &str;
    httpequiv, "httpEquiv" => &str;
    integrity, "integrity" => &str;
    keyparams, "keyParams" => &str;
    keytype, "keyType" => &str;
    kind, "kind" => &str;
    label, "label" => &str;
    list, "list" => &str;
    low, "low" => f64;
    manifest, "manifest" => &str;
    marginheight, "marginHeight" => f64;
    marginwidth, "marginWidth" => f64;
    max, "max" => f64;
    maxlength, "maxLength" => f64;
    media, "media" => &str;
    mediagroup, "mediaGroup" => &str;
    method, "method" => &str;
    min, "min" => impl Into<JsValue>;
    minlength, "minLength" => f64;
    multiple, "multiple" => bool;
    muted, "muted" => bool;
    name, "name" => &str;
    nonce, "nonce" => &str;
    novalidate, "noValidate" => bool;
    open, "open" => bool;
    optimum, "optimum" => f64;
    pattern, "pattern" => &str;
    playsinline, "playsInline" => bool;
    poster, "poster" => &str;
    preload, "preload" => &str;
    readonly, "readOnly" => bool;
    rel, "rel" => &str;
    required, "required" => bool;
    reversed, "reversed" => bool;
    rows, "rows" => u32;
    rowspan, "rowSpan" => u32;
    sandbox, "sandbox" => &str;
    scope, "scope" => &str;
    scoped, "scoped" => bool;
    scrolling, "scrolling" => &str;
    seamless, "seamless" => bool;
    selected, "selected" => bool;
    shape, "shape" => &str;
    size, "size" => f64;
    sizes, "sizes" => &str;
    span, "span" => u32;
    src, "src" => &str;
    srcdoc, "srcDoc" => &str;
    srclang, "srcLang" => &str;
    srcset, "srcSet" => &str;
    start, "start" => f64;
    step, "step" => impl Into<JsValue>;
    summary, "summary" => &str;
    target, "target" => &str;
    usemap, "useMap" => &str;
    value, "value" => impl Into<JsValue>;
    width, "width" => impl Into<JsValue>;
    wmode, "wmode" => &str;
    wrap, "wrap" => &str;
  }
}
