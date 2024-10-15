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
pub(crate) static META_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("meta").unwrap());
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

pub(crate) static META_TITLE_KEYS: &[&str] = &[
    "dc:title",
    "dcterm:title",
    "og:title",
    "weibo:article:title",
    "weibo:webpage:title",
    "title",
    "twitter:title",
    "parsely-title",
];


pub (crate) static META_IMAGE_KEYS: &[&str] = &["og:image", "image","twitter:image" ];
pub (crate) static META_MOD_TIME_KEYS: &[&str] = &["article:modified_time", "dcterms.modifie" ];
pub (crate) static META_PUB_TIME_KEYS: &[&str] = &["article:published_time", "dcterms.available", "dcterms.created", "dcterms.issued", "parsely-pub-date", "weibo:article:create_at" ];
pub (crate) static META_BYLINE_KEYS: &[&str] = &["dc:creator", "dcterms:creator", "author","parsely-author" ];
pub (crate) static META_EXCERPT_KEYS: &[&str] = &["dc:description", "dcterm:description", "og:description", "weibo:article:description", "weibo:webpage:description", "description", "twitter:description"];


//TODO: replace \s+
pub(crate) static RX_TOKENIZE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?i)\W+"#).unwrap());
pub(crate) static RX_CDATA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^\s*<!\[CDATA\[|\]\]>\s*$"#).unwrap());

pub(crate) static RX_SCHEMA_ORG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)^https?://schema\.org/?$"#).unwrap());
pub(crate) static RX_TITLE_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\|\-\\—/>»]"#).unwrap());
pub(crate) static RX_TITLE_ANY_SEP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\|\-\\—/>»]+"#).unwrap());
pub(crate) static RX_HIERARCHY_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\\/>»]"#).unwrap());

//TODO: replace these with &[&str], because there is no reason to use regex here.

pub(crate) static RX_META_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)^\s*(?:(dc|dcterm|og|twitter|parsely|weibo:(article|webpage))\s*[-\.:]\s*)?(author|creator|pub-date|description|title|site_name)\s*$"#).unwrap()
});
pub(crate) static RX_META_PROPERTY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)\s*(article|dc|dcterm|og|twitter)\s*:\s*(author|creator|description|published_time|title|site_name)\s*"#).unwrap()
});
pub(crate) static RX_JSONLD_ARTICLE_TYPES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)^Article|AdvertiserContentArticle|NewsArticle|AnalysisNewsArticle|AskPublicNewsArticle|BackgroundNewsArticle|OpinionNewsArticle|ReportageNewsArticle|ReviewNewsArticle|Report|SatiricalArticle|ScholarlyArticle|MedicalScholarlyArticle|SocialMediaPosting|BlogPosting|LiveBlogPosting|DiscussionForumPosting|TechArticle|APIReference$"#).unwrap()
});
