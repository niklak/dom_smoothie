use dom_query::Matcher;
use once_cell::sync::Lazy;
use regex::Regex;
pub(crate) static TITLE_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new("head title").unwrap());

pub(crate) static SCRIPT_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("script, noscript").unwrap());

pub(crate) static HTML_LANG_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("html[lang]").unwrap());
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
pub(crate) static ROLES_MATCHER: Lazy<Matcher> = Lazy::new(|| Matcher::new(r#"*[role]"#).unwrap());
pub(crate) static DIALOGS_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"*[aria-modal="true"][role="dialog"]"#).unwrap());
pub(crate) static EMPTY_SECTION_MATCHER: Lazy<Matcher> = Lazy::new(|| {
    Matcher::new(r#"div:empty,section:empty,header:empty,h1:empty,h2:empty,h3:empty,h4:empty,h5:empty,h6:empty"#).unwrap()
});

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

pub(crate) static META_IMAGE_KEYS: &[&str] = &["og:image", "image", "twitter:image"];
pub(crate) static META_MOD_TIME_KEYS: &[&str] = &["article:modified_time", "dcterms.modifie"];
pub(crate) static META_PUB_TIME_KEYS: &[&str] = &[
    "article:published_time",
    "dcterms.available",
    "dcterms.created",
    "dcterms.issued",
    "parsely-pub-date",
    "weibo:article:create_at",
];
pub(crate) static META_BYLINE_KEYS: &[&str] =
    &["dc:creator", "dcterms:creator", "author", "parsely-author"];
pub(crate) static META_EXCERPT_KEYS: &[&str] = &[
    "dc:description",
    "dcterm:description",
    "og:description",
    "weibo:article:description",
    "weibo:webpage:description",
    "description",
    "twitter:description",
];

pub(crate) static UNLIKELY_ROLES: &[&str] = &[
    "menu",
    "menubar",
    "complementary",
    "navigation",
    "alert",
    "alertdialog",
    "dialog",
];

pub(crate) static BLOCK_ELEMS: &[&str] = &[
    "blockquote",
    "dl",
    "div",
    "img",
    "ol",
    "p",
    "pre",
    "table",
    "ul",
];

pub(crate) static ALTER_TO_DIV_EXCEPTIONS: &[&str] = &[
    "div", "article", "section", "p", "ol", "ul"
];

pub(crate) static DEFAULT_TAGS_TO_SCORE: &[&str] =
    &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

//TODO: replace \s+
pub(crate) static RX_TOKENIZE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?i)\W+"#).unwrap());
pub(crate) static RX_STYLE_DISPLAY_NONE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)display\s*:\s*none|visibility\s*:\s*hidden"#).unwrap());
pub(crate) static RX_CDATA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^\s*<!\[CDATA\[|\]\]>\s*$"#).unwrap());

pub(crate) static RX_SCHEMA_ORG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)^https?://schema\.org/?$"#).unwrap());
pub(crate) static RX_TITLE_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\|\-\\—/>»]"#).unwrap());
pub(crate) static RX_TITLE_ANY_SEP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\|\-\\—/>»]+"#).unwrap());
pub(crate) static RX_HIERARCHY_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\\/>»]"#).unwrap());
pub(crate) static RX_BYLINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)byline|author|dateline|writtenby|p-author"#).unwrap());

pub(crate) static RX_UNLIKELY_CANDIDATES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|pagination|pager|popup|yom-remote"#).unwrap()
});
pub(crate) static RX_MAYBE_CANDIDATES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"and|article|body|column|content|main|shadow"#).unwrap());

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
pub(crate) static RX_HAS_CONTENT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\S$"#).unwrap());
pub(crate) static RX_HASH_URL: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^#.+"#).unwrap());
pub(crate) static RX_COMMAS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\u002C|\u060C|\uFE50|\uFE10|\uFE11|\u2E41|\u2E34|\u2E32|\uFF0C"#).unwrap()
});
pub(crate) static RX_CLASSES_NEGATIVE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"-ad-|hidden|^hid$| hid$| hid |^hid |banner|combx|comment|com-|contact|footer|gdpr|masthead|media|meta|outbrain|promo|related|scroll|share|shoutbox|sidebar|skyscraper|sponsor|shopping|tags|widget"#).unwrap()
});
pub(crate) static RX_CLASSES_POSITIVE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"article|body|content|entry|hentry|h-entry|main|page|pagination|post|text|blog|story"#,
    )
    .unwrap()
});
pub(crate) static RX_SENTENCE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.( |$)").unwrap());
pub(crate) static DEFAULT_N_TOP_CANDIDATES: usize = 5;
pub(crate) static MINIMUM_TOP_CANDIDATES: usize = 3;
pub(crate) static DEFAULT_CHAR_THRESHOLD: usize = 500;