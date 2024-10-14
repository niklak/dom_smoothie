use once_cell::sync::Lazy;
use dom_query::Matcher;

pub(crate) static TITLE_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("head title").unwrap());

pub(crate) static SCRIPT_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("script, noscript").unwrap());

pub(crate) static STYLE_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("style").unwrap());

pub(crate) static FONT_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("font").unwrap());

pub(crate) static BR_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("br").unwrap());

pub (crate) static PHRASING_ELEMS: &[&str] = &[
    // "canvas", "iframe", "svg", "video",
    "abbr",
    "audio",
    "b",
    "bdo",
    "br",
    "button",
    "cite",
    "code",
    "data",
    "datalist",
    "dfn",
    "em",
    "embed",
    "i",
    "img",
    "input",
    "kbd",
    "label",
    "mark",
    "math",
    "meter",
    "noscript",
    "object",
    "output",
    "progress",
    "q",
    "ruby",
    "samp",
    "script",
    "select",
    "small",
    "span",
    "strong",
    "sub",
    "sup",
    "textarea",
    "time",
    "var",
    "wbr",
  ];