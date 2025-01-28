use super::Tag;
use crate::JsComponent;
use js_sys::JsString;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

macro_rules! impl_attr {
  { $( $attr:ident, $key:ident; )* } => {
    $(
      #[allow(missing_docs)]
      pub fn $attr(self, value: impl Into<JsValue>) -> Self {
        $key.with(|key|
          self.prop(key.clone(), value)
        )
      }
    )*
  };
}

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
  // Standard HTML Attributes
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ACCESSKEY: JsString = "accessKey";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CLASSNAME: JsString = "className";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CONTENTEDITABLE: JsString = "contentEditable";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CONTEXTMENU: JsString = "contextMenu";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DIR: JsString = "dir";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DRAGGABLE: JsString = "draggable";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HIDDEN: JsString = "hidden";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ID: JsString = "id";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static LANG: JsString = "lang";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static PLACEHOLDER: JsString = "placeholder";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SLOT: JsString = "slot";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SPELLCHECK: JsString = "spellCheck";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static TABINDEX: JsString = "tabIndex";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static TITLE: JsString = "title";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static TRANSLATE: JsString = "translate";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static RADIOGROUP: JsString = "radioGroup";

  // WAI-ARIA
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ROLE: JsString = "role";

  // RDFa Attributes
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ABOUT: JsString = "about";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DATATYPE: JsString = "datatype";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static INLIST: JsString = "inlist";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static PREFIX: JsString = "prefix";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static PROPERTY: JsString = "property";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static RESOURCE: JsString = "resource";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static VOCAB: JsString = "vocab";

  // Living Standard
  #[wasm_bindgen(thread_local_v2, static_string)]
  static INPUTMODE: JsString = "inputMode";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static IS: JsString = "is";

  // Standard HTML Attributes
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ACCEPT: JsString = "accept";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ACCEPTCHARSET: JsString = "acceptCharset";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ACTION: JsString = "action";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ALLOWFULLSCREEN: JsString = "allowFullScreen";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ALLOWTRANSPARENCY: JsString = "allowTransparency";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ALT: JsString = "alt";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static AUTOCOMPLETE: JsString = "autoComplete";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static AUTOFOCUS: JsString = "autoFocus";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static AUTOPLAY: JsString = "autoPlay";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CAPTURE: JsString = "capture";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CELLPADDING: JsString = "cellPadding";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CELLSPACING: JsString = "cellSpacing";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHALLENGE: JsString = "challenge";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHARSET: JsString = "charSet";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CHECKED: JsString = "checked";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CITE: JsString = "cite";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CLASSID: JsString = "classID";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static COLS: JsString = "cols";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static COLSPAN: JsString = "colSpan";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CONTENT: JsString = "content";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CONTROLS: JsString = "controls";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static COORDS: JsString = "coords";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static CROSSORIGIN: JsString = "crossOrigin";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DANGEROUSLY_SET_INNER_HTML: JsString = "dangerouslySetInnerHTML";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DATA: JsString = "data";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DATETIME: JsString = "dateTime";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DEFAULT: JsString = "default";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DEFER: JsString = "defer";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DISABLED: JsString = "disabled";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static DOWNLOAD: JsString = "download";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ENCTYPE: JsString = "encType";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FORM: JsString = "form";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FORMACTION: JsString = "formAction";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FORMENCTYPE: JsString = "formEncType";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FORMMETHOD: JsString = "formMethod";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FORMNOVALIDATE: JsString = "formNoValidate";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FORMTARGET: JsString = "formTarget";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static FRAMEBORDER: JsString = "frameBorder";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HEADERS: JsString = "headers";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HEIGHT: JsString = "height";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HIGH: JsString = "high";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HREF: JsString = "href";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HREFLANG: JsString = "hrefLang";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HTML_FOR: JsString = "htmlFor";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HTML_TYPE: JsString = "type";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static HTTPEQUIV: JsString = "httpEquiv";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static INTEGRITY: JsString = "integrity";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static KEYPARAMS: JsString = "keyParams";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static KEYTYPE: JsString = "keyType";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static KIND: JsString = "kind";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static LABEL: JsString = "label";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static LIST: JsString = "list";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static LOW: JsString = "low";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MANIFEST: JsString = "manifest";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MARGINHEIGHT: JsString = "marginHeight";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MARGINWIDTH: JsString = "marginWidth";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MAX: JsString = "max";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MAXLENGTH: JsString = "maxLength";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MEDIA: JsString = "media";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MEDIAGROUP: JsString = "mediaGroup";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static METHOD: JsString = "method";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MIN: JsString = "min";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MINLENGTH: JsString = "minLength";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MULTIPLE: JsString = "multiple";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static MUTED: JsString = "muted";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static NAME: JsString = "name";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static NONCE: JsString = "nonce";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static NOVALIDATE: JsString = "noValidate";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static OPEN: JsString = "open";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static OPTIMUM: JsString = "optimum";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static PATTERN: JsString = "pattern";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static PLAYSINLINE: JsString = "playsInline";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static POSTER: JsString = "poster";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static PRELOAD: JsString = "preload";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static READONLY: JsString = "readOnly";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static REL: JsString = "rel";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static REQUIRED: JsString = "required";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static REVERSED: JsString = "reversed";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ROWS: JsString = "rows";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static ROWSPAN: JsString = "rowSpan";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SANDBOX: JsString = "sandbox";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SCOPE: JsString = "scope";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SCOPED: JsString = "scoped";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SCROLLING: JsString = "scrolling";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SEAMLESS: JsString = "seamless";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SELECTED: JsString = "selected";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SHAPE: JsString = "shape";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SIZE: JsString = "size";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SIZES: JsString = "sizes";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SPAN: JsString = "span";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SRC: JsString = "src";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SRCDOC: JsString = "srcDoc";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SRCLANG: JsString = "srcLang";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SRCSET: JsString = "srcSet";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static START: JsString = "start";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static STEP: JsString = "step";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static STYLE: JsString = "style";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static SUMMARY: JsString = "summary";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static TARGET: JsString = "target";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static USEMAP: JsString = "useMap";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static VALUE: JsString = "value";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static WIDTH: JsString = "width";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static WMODE: JsString = "wmode";
  #[wasm_bindgen(thread_local_v2, static_string)]
  static WRAP: JsString = "wrap";
}

impl JsComponent<Tag> {
  impl_attr! {
    // Standard HTML Attributes
    accesskey, ACCESSKEY;
    classname, CLASSNAME;
    contenteditable, CONTENTEDITABLE;
    contextmenu, CONTEXTMENU;
    dir, DIR;
    draggable, DRAGGABLE;
    hidden, HIDDEN;
    id, ID;
    lang, LANG;
    placeholder, PLACEHOLDER;
    slot, SLOT;
    spellcheck, SPELLCHECK;
    tabindex, TABINDEX;
    title, TITLE;
    translate, TRANSLATE;
    radiogroup, RADIOGROUP;

    // WAI-ARIA
    role, ROLE;

    // RDFa Attributes
    about, ABOUT;
    datatype, DATATYPE;
    inlist, INLIST;
    prefix, PREFIX;
    property, PROPERTY;
    resource, RESOURCE;
    vocab, VOCAB;

    // Living Standard
    inputmode, INPUTMODE;
    is, IS;

    // Standard HTML Attributes
    accept, ACCEPT;
    acceptcharset, ACCEPTCHARSET;
    action, ACTION;
    allowfullscreen, ALLOWFULLSCREEN;
    allowtransparency, ALLOWTRANSPARENCY;
    alt, ALT;
    autocomplete, AUTOCOMPLETE;
    autofocus, AUTOFOCUS;
    autoplay, AUTOPLAY;
    capture, CAPTURE;
    cellpadding, CELLPADDING;
    cellspacing, CELLSPACING;
    challenge, CHALLENGE;
    charset, CHARSET;
    checked, CHECKED;
    cite, CITE;
    classid, CLASSID;
    cols, COLS;
    colspan, COLSPAN;
    content, CONTENT;
    controls, CONTROLS;
    coords, COORDS;
    crossorigin, CROSSORIGIN;
    data, DATA;
    datetime, DATETIME;
    default, DEFAULT;
    defer, DEFER;
    disabled, DISABLED;
    download, DOWNLOAD;
    enctype, ENCTYPE;
    form, FORM;
    formaction, FORMACTION;
    formenctype, FORMENCTYPE;
    formmethod, FORMMETHOD;
    formnovalidate, FORMNOVALIDATE;
    formtarget, FORMTARGET;
    frameborder, FRAMEBORDER;
    headers, HEADERS;
    height, HEIGHT;
    high, HIGH;
    href, HREF;
    hreflang, HREFLANG;
    html_for, HTML_FOR;
    html_type, HTML_TYPE;
    httpequiv, HTTPEQUIV;
    integrity, INTEGRITY;
    keyparams, KEYPARAMS;
    keytype, KEYTYPE;
    kind, KIND;
    label, LABEL;
    list, LIST;
    low, LOW;
    manifest, MANIFEST;
    marginheight, MARGINHEIGHT;
    marginwidth, MARGINWIDTH;
    max, MAX;
    maxlength, MAXLENGTH;
    media, MEDIA;
    mediagroup, MEDIAGROUP;
    method, METHOD;
    min, MIN;
    minlength, MINLENGTH;
    multiple, MULTIPLE;
    muted, MUTED;
    name, NAME;
    nonce, NONCE;
    novalidate, NOVALIDATE;
    open, OPEN;
    optimum, OPTIMUM;
    pattern, PATTERN;
    playsinline, PLAYSINLINE;
    poster, POSTER;
    preload, PRELOAD;
    readonly, READONLY;
    rel, REL;
    required, REQUIRED;
    reversed, REVERSED;
    rows, ROWS;
    rowspan, ROWSPAN;
    sandbox, SANDBOX;
    scope, SCOPE;
    scoped, SCOPED;
    scrolling, SCROLLING;
    seamless, SEAMLESS;
    selected, SELECTED;
    shape, SHAPE;
    size, SIZE;
    sizes, SIZES;
    span, SPAN;
    src, SRC;
    srcdoc, SRCDOC;
    srclang, SRCLANG;
    srcset, SRCSET;
    start, START;
    step, STEP;
    summary, SUMMARY;
    target, TARGET;
    usemap, USEMAP;
    value, VALUE;
    width, WIDTH;
    wmode, WMODE;
    wrap, WRAP;
  }
}
