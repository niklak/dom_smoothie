use dom_query::Matcher;
use once_cell::sync::Lazy;
use regex::Regex;
pub(crate) static TITLE_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("head title").unwrap());

pub(crate) static SCRIPT_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("script, noscript").unwrap());

pub(crate) static STYLE_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("style").unwrap());
pub(crate) static FONT_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("font").unwrap());
pub(crate) static BR_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("br").unwrap());
pub(crate) static IMG_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("img").unwrap());
pub(crate) static UNWANTED_A_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"a[href^="javascript:"]"#).unwrap());
pub(crate) static JSONLD_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"script[type="application/ld+json"]"#).unwrap());
pub(crate) static HEADINGS_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new(r#"h1,h2"#).unwrap());

pub(crate) static PHRASING_ELEMS: &[&str] = &[
    // "canvas", "iframe", "svg", "video",
    "abbr", "audio", "b", "bdo", "br", "button", "cite", "code", "data", "datalist", "dfn", "em",
    "embed", "i", "img", "input", "kbd", "label", "mark", "math", "meter", "noscript", "object",
    "output", "progress", "q", "ruby", "samp", "script", "select", "small", "span", "strong",
    "sub", "sup", "textarea", "time", "var", "wbr",
];

//TODO: replace \s+
pub(crate) static RX_TOKENIZE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?i)\W+"#).unwrap());

pub(crate) static RX_SCHEMA_ORG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)^https?://schema\.org/?$"#).unwrap());
pub(crate) static RX_TITLE_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\|\-\\—/>»]"#).unwrap());
pub(crate) static RX_TITLE_ANY_SEP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\|\-\\—/>»]+"#).unwrap());
pub(crate) static RX_HIERARCHY_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\\/>»]"#).unwrap());
pub(crate) static RX_JSONLD_ARTICLE_TYPES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)^Article|AdvertiserContentArticle|NewsArticle|AnalysisNewsArticle|AskPublicNewsArticle|BackgroundNewsArticle|OpinionNewsArticle|ReportageNewsArticle|ReviewNewsArticle|Report|SatiricalArticle|ScholarlyArticle|MedicalScholarlyArticle|SocialMediaPosting|BlogPosting|LiveBlogPosting|DiscussionForumPosting|TechArticle|APIReference$"#).unwrap()
});
