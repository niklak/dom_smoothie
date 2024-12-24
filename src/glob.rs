use dom_query::Matcher;
use once_cell::sync::Lazy;
use regex::Regex;
pub(crate) static MATCHER_TITLE: Lazy<Matcher> = Lazy::new(|| Matcher::new("head title").unwrap());
pub(crate) static MATCHER_SCRIPT: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("script, noscript").unwrap());
pub(crate) static MATCHER_HTML_LANG: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("html[lang]").unwrap());
pub(crate) static MATCHER_STYLE: Lazy<Matcher> = Lazy::new(|| Matcher::new("style").unwrap());
pub(crate) static MATCHER_FONT: Lazy<Matcher> = Lazy::new(|| Matcher::new("font").unwrap());
pub(crate) static MATCHER_BR: Lazy<Matcher> = Lazy::new(|| Matcher::new("br").unwrap());
pub(crate) static MATCHER_IMG: Lazy<Matcher> = Lazy::new(|| Matcher::new("img").unwrap());
pub(crate) static MATCHER_META: Lazy<Matcher> = Lazy::new(|| Matcher::new("meta").unwrap());
pub(crate) static MATCHER_JS_LINK: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"a[href^="javascript:"]"#).unwrap());
pub(crate) static MATCHER_JSONLD: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"script[type="application/ld+json"]"#).unwrap());
pub(crate) static MATCHER_HEADING: Lazy<Matcher> = Lazy::new(|| Matcher::new(r#"h1,h2"#).unwrap());
pub(crate) static MATCHER_DIALOGS: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"*[aria-modal="true"][role="dialog"]"#).unwrap());
pub(crate) static MATCHER_BYLINE: Lazy<Matcher> =
    Lazy::new(|| Matcher::new(r#"[rel="author"],[itemprop*="author"]"#).unwrap());

pub(crate) static MATCHER_A: Lazy<Matcher> = Lazy::new(|| Matcher::new("a").unwrap());
pub(crate) static MATCHER_BR_HR: Lazy<Matcher> = Lazy::new(|| Matcher::new("br,hr").unwrap());
pub(crate) static MATCHER_SOURCES: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("img,picture,figure,video,audio,sources").unwrap());
pub(crate) static MATCHER_BASE: Lazy<Matcher> = Lazy::new(|| Matcher::new("base[href]").unwrap());
pub(crate) static MATCHER_DIR: Lazy<Matcher> = Lazy::new(|| Matcher::new("*[dir]").unwrap());
pub(crate) static MATCHER_P: Lazy<Matcher> = Lazy::new(|| Matcher::new("p").unwrap());
pub(crate) static MATCHER_EMBEDS: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("object,embed,iframe").unwrap());

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

pub(crate) static ALTER_TO_DIV_EXCEPTIONS: &[&str] =
    &["div", "article", "section", "p", "ol", "ul"];

pub(crate) static DEFAULT_TAGS_TO_SCORE: &[&str] =
    &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

pub(crate) static TAGS_WITH_CONTENT: &[&str] = &[
    "div", "section", "header", "h1", "h2", "h3", "h4", "h5", "h6",
];

pub(crate) static PRESENTATIONAL_ATTRIBUTES: &[&str] = &[
    "align",
    "background",
    "bgcolor",
    "border",
    "cellpadding",
    "cellspacing",
    "frame",
    "hspace",
    "rules",
    "style",
    "valign",
    "vspace",
];

pub(crate) static EMBED_ELEMENTS: &[&str] = &["object", "embed", "iframe"];

pub(crate) static DEPRECATED_SIZE_ATTRIBUTE_ELEMS: &[&str] = &["table", "th", "td", "hr", "pre"];

pub(crate) static RX_STYLE_DISPLAY_NONE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)display\s*:\s*none|visibility\s*:\s*hidden"#).unwrap());
pub(crate) static RX_CDATA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^\s*<!\[CDATA\[|\]\]>\s*$"#).unwrap());

pub(crate) static RX_SCHEMA_ORG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)^https?://schema\.org/?$"#).unwrap());
pub(crate) static RX_TITLE_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r#" [\|\-\\/>»] "#).unwrap());
pub(crate) static RX_TITLE_W_LAST: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(.*)[\|\-\\/>»] .*"#).unwrap());
pub(crate) static RX_TITLE_W_FIRST: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[^\|\-\\/>»]*[\|\-\\/>»](.*)"#).unwrap());
pub(crate) static RX_TITLE_ANY_SEP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\|\-\\/>»]+"#).unwrap());
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

pub(crate) static RX_HASH_URL: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^#.+"#).unwrap());
pub(crate) static RX_COMMAS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\u002C|\u060C|\uFE50|\uFE10|\uFE11|\u2E41|\u2E34|\u2E32|\uFF0C"#).unwrap()
});
pub(crate) static RX_CLASSES_NEGATIVE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)-ad-|hidden|^hid$| hid$| hid |^hid |banner|combx|comment|com-|contact|footer|gdpr|masthead|media|meta|outbrain|promo|related|scroll|share|shoutbox|sidebar|skyscraper|sponsor|shopping|tags|widget"#).unwrap()
});
pub(crate) static RX_CLASSES_POSITIVE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?i)article|body|content|entry|hentry|h-entry|main|page|pagination|post|text|blog|story"#,
    )
    .unwrap()
});
pub(crate) static RX_SENTENCE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.( |$)").unwrap());
pub(crate) static RX_VIDEO_ATTRS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"//(www\.)?((dailymotion|youtube|youtube-nocookie|player\.vimeo|v\.qq)\.com|(archive|upload\.wikimedia)\.org|player\.twitch\.tv)"#).unwrap()
});
pub(crate) static RX_BASE64_URL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)^data:\s*([^\s;,]+)\s*;\s*base64\s*"#).unwrap());
pub(crate) static RX_IMG_ATTR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i).(jpg|jpeg|png|webp)"#).unwrap());
pub(crate) static RX_IMG_ATTR_TO_SRC: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)^\s*\S+\.(jpg|jpeg|png|webp)\S*\s*$"#).unwrap());
pub(crate) static RX_IMG_ATTR_TO_SRCSET: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i).(jpg|jpeg|png|webp)\s+\d"#).unwrap());

pub(crate) static RX_AD_WORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)^(ad(vertising|vertisement)?|pub(licité)?|werb(ung)?|广告|Реклама|Anuncio)$"#)
        .unwrap()
});
pub(crate) static RX_LOADING_WORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)^((loading|正在加载|Загрузка|chargement|cargando)(…|\.\.\.)?)$"#).unwrap()
});
pub(crate) static RX_SHARE_ELEMENTS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)(\b|_)(share|sharedaddy)(\b|_)"#).unwrap());

pub(crate) static DEFAULT_N_TOP_CANDIDATES: usize = 5;
pub(crate) static MINIMUM_TOP_CANDIDATES: usize = 3;
pub(crate) static DEFAULT_CHAR_THRESHOLD: usize = 500;
pub(crate) static SCORE_ATTR: &str = "data-readability-score";
