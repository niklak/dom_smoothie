//! Functions below replace regex-based validation with explicit string matching
//! for better maintainability and performance

use tendril::StrTendril;

use dom_query::Node;

use crate::glob::*;

pub(crate) fn is_invisible_style(node: &Node) -> bool {
    if let Some(mut style) = node.attr("style") {
        style.make_ascii_lowercase();
        return style_has_kv(&style, "display", "none")
            || style_has_kv(&style, "visibility", "hidden");
    }
    false
}

fn style_has_kv(style: &str, key: &str, val: &str) -> bool {
    if let Some(pos) = style.find(key) {
        let mut rest = &style[pos..];
        if let Some(pos) = rest.find(':') {
            rest = &rest[pos + 1..];
        } else {
            return false;
        }
        if let Some(pos) = rest.find(';') {
            rest = &rest[..pos];
        }
        rest = rest.trim_start();
        if let Some(pos) = rest.find(char::is_whitespace) {
            rest = &rest[..pos];
        }
        return rest.trim_end() == val;
    }
    false
}

pub(crate) fn strip_cdata(content: &StrTendril) -> &str {
    if let Some(rest) = content.trim_start().strip_prefix("<![CDATA[") {
        return rest.split("]]>").next().unwrap_or(rest);
    }
    content
}

pub(crate) fn is_schema_org_url(url: &str) -> bool {
    let trimmed_url = url.trim_end_matches('/');
    trimmed_url.ends_with(SCHEMA_ORG_SFX)
        && (trimmed_url.starts_with(HTTP_PFX) || trimmed_url.starts_with(HTTPS_PFX))
}

pub(crate) fn is_video_url(haystack: &str) -> bool {
    VIDEO_DOMAINS.iter().any(|&p| {
        if let Some(pos) = haystack.find(p) {
            if pos > 1 && &haystack[pos - PROTOCOL_PFX_LEN..pos] == PROTOCOL_PFX
                || pos > 5 && &haystack[pos - WWW_PFX_LEN..pos] == WWW_PFX
            {
                return true;
            }
        }
        false
    })
}

pub(crate) fn is_sentence(text: &str) -> bool {
    text.ends_with('.') || text.contains(". ")
}

pub(crate) fn contains_one_of_words(haystack: &str, words: &[&str]) -> bool {
    haystack
        .split_whitespace()
        .any(|word| words.contains(&word))
}

#[inline]
pub(crate) fn is_img_attr_to_srcset(s: &str) -> bool {
    for ext in IMG_EXT {
        let mut start = 0;
        while let Some(pos) = s[start..].find(ext) {
            let idx = start + pos + ext.len();
            if idx < s.len() - 1 {
                let bytes = s.as_bytes();
                if bytes[idx].is_ascii_whitespace() && bytes[idx + 1].is_ascii_digit() {
                    return true;
                }
            }
            start = idx;
        }
    }
    false
}

#[inline]
pub(crate) fn is_img_attr_to_src(s: &str) -> bool {
    s.trim()
        .split('.')
        .skip(1)
        .any(|part| IMG_EXT.iter().any(|ext| part.starts_with(&ext[1..])))
}

pub(crate) fn truncate_title_last(title: &str) -> Option<&str> {
    // This is not a perfect, but behaves as like RX_TITLE_W_LAST
    if let Some((delim_pos, sep)) = title
        .char_indices()
        .rev()
        .find(|(_, c)| TITLE_SEPARATORS.contains(c))
    {
        let next_char = title.get(delim_pos + sep.len_utf8()..)?.chars().next()?;
        if next_char == ' ' {
            return title.get(..delim_pos).map(str::trim);
        }
    }
    None
}

pub(crate) fn truncate_title_first(title: &str) -> Option<&str> {
    // This is not a perfect, but behaves as like RX_TITLE_W_LAST
    if let Some((delim_pos, sep)) = title
        .char_indices()
        .find(|(_, c)| TITLE_SEPARATORS.contains(c))
    {
        let next_char = title.get(delim_pos + sep.len_utf8()..)?.chars().next()?;
        if next_char == ' ' {
            return title.get(delim_pos + sep.len_utf8()..).map(str::trim);
        }
    }
    None
}

pub(crate) fn is_meta_name(name: &str) -> bool {
    if let Some((prefix, key)) = name.split_once(META_NAME_SEP) {
        return META_NAME_PREFIXES.contains(&prefix) && META_NAME_KEYS.contains(&key);
    }
    META_NAME_KEYS.contains(&name)
}

pub(crate) fn meta_property_name(property: &str) -> Option<&str> {
    for part in property.split_whitespace() {
        if let Some(pos_r) = part.rfind(':') {
            let key = &part[pos_r + 1..];
            if !META_PROPERTY_KEYS.contains(&key) {
                continue;
            }
            let pre_pos = if let Some(pos_l) = part[..pos_r].find(':') {
                pos_l + 1
            } else {
                0
            };
            let pre = &part[pre_pos..pos_r];
            if META_PROPERTY_PREFIXES.contains(&pre) {
                return Some(&part[pre_pos..]);
            }
        }
    }
    None
}

pub(crate) fn is_loading_word(text: &str) -> bool {
    let trimmed = text.trim_end_matches(['…', '.']);
    LOADING_WORDS.contains(trimmed)
}

pub(crate) fn contains_share_elements(value: &str) -> bool {
    let lower_value = value.to_ascii_lowercase();
    lower_value
        .split([' ', '_'])
        .any(|word| SHARE_WORDS.contains(word))
}

pub(crate) fn split_base64_url(src: &str) -> Option<(&str, &str)> {
    if let Some(rest) = src.strip_prefix("data:") {
        if let Some(pos) = rest.find(BASE64_MARKER) {
            let image_type = &rest[..pos];
            let image_data = &rest[pos + BASE64_MARKER_LEN..];
            if image_data.is_empty() {
                return None;
            }
            return Some((image_type, image_data));
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_meta_property_name() {
        assert_eq!(
            meta_property_name("og:article:author"),
            Some("article:author")
        );
        assert_eq!(meta_property_name("x:title og:title"), Some("og:title"));
    }

    #[test]
    fn test_is_meta_name() {
        assert!(is_meta_name("author"));
        assert!(is_meta_name("dc:title"));
        assert!(is_meta_name("dc:title"));
        assert!(!is_meta_name("dc:mod-date"));
        assert!(is_meta_name("dc:pub-date"));
    }

    #[test]
    fn test_truncate_title_first() {
        let title1 = "Lazy Load with Alt includes jpg/png/webp extensions";
        assert_eq!(truncate_title_first(title1), None);
        let title2 = "Some Title | Some Extra Info ";
        assert_eq!(truncate_title_first(title2), Some("Some Extra Info"));
    }

    #[test]
    fn test_truncate_title_last() {
        let orig_title = "Lazy Load with Alt includes jpg/png/webp extensions";
        assert_eq!(truncate_title_last(orig_title), None,);

        let orig_title = "Lazy Load with Alt includes jpg / png / webp extensions";
        assert_eq!(
            truncate_title_last(orig_title),
            Some("Lazy Load with Alt includes jpg / png")
        );
    }

    #[test]
    fn test_is_img_attr_to_src() {
        let val = "https://static01.nyt.com/images/2019/02/15/nyregion/
        00winterutilitiesOAK11/merlin_94083158_9e622a52-ec2f-4fbd-845c-
        5d530e94bc82-articleLarge.jpg?quality=90&amp;auto=webp";
        assert!(is_img_attr_to_src(val));
    }

    #[test]
    fn test_contains_one_of_words() {
        assert!(contains_one_of_words(
            "something hid",
            CLASSES_NEGATIVE_WORDS
        ));
        assert!(contains_one_of_words(
            "something hid another",
            CLASSES_NEGATIVE_WORDS
        ));
        assert!(contains_one_of_words(
            "hid something",
            CLASSES_NEGATIVE_WORDS
        ));
        assert!(!contains_one_of_words(
            "something hidden",
            CLASSES_NEGATIVE_WORDS
        ));
    }

    #[test]
    fn test_strip_cdata() {
        // Test valid CDATA
        let content = StrTendril::from_slice("<![CDATA[test content]]>");
        assert_eq!(strip_cdata(&content), "test content");

        // Test missing closing marker
        let content = StrTendril::from_slice("<![CDATA[test content");
        assert_eq!(strip_cdata(&content), "test content");

        // Test no CDATA
        let content = StrTendril::from_slice("test content");
        assert_eq!(strip_cdata(&content), "test content");

        // Test empty content
        let content = StrTendril::from_slice("");
        assert_eq!(strip_cdata(&content), "");

        // Test whitespace
        let content = StrTendril::from_slice("  <![CDATA[test content]]>");
        assert_eq!(strip_cdata(&content), "test content");
    }

    #[test]
    fn test_is_schema_org_url() {
        // Valid URLs
        assert!(is_schema_org_url("http://schema.org"));
        assert!(is_schema_org_url("https://schema.org"));
        assert!(is_schema_org_url("http://schema.org/"));
        assert!(is_schema_org_url("https://schema.org/"));
        assert!(is_schema_org_url("http://schema.org////")); // multiple trailing slashes

        // Invalid URLs
        assert!(!is_schema_org_url("ftp://schema.org"));
        assert!(!is_schema_org_url("//schema.org"));
        assert!(!is_schema_org_url("schema.org"));
        assert!(!is_schema_org_url("http://schemaXorg"));
        assert!(!is_schema_org_url(""));
    }

    #[test]
    fn test_is_video_url() {
        // Valid URLs with protocol prefix
        assert!(is_video_url("//youtube.com/watch?v=123"));
        assert!(is_video_url("//player.vimeo.com/video/123"));
        assert!(is_video_url("//dailymotion.com/video/123"));
        assert!(is_video_url("//youtube-nocookie.com/embed/123"));
        assert!(is_video_url("//v.qq.com/video/123"));
        assert!(is_video_url("//archive.org/video/123"));
        assert!(is_video_url("//upload.wikimedia.org/video/123"));
        assert!(is_video_url("//player.twitch.tv/video/123"));

        // Valid URLs with www prefix
        assert!(is_video_url("//www.youtube.com/watch?v=123"));
        assert!(is_video_url("//www.dailymotion.com/video/123"));

        // Invalid URLs
        assert!(!is_video_url("youtube.com/watch?v=123")); // missing prefix
        assert!(!is_video_url("http://notvideo.com/youtube.com")); // video domain in path
        assert!(!is_video_url("//youtubeXcom/watch?v=123")); // invalid domain
        assert!(!is_video_url("//www.notvideo.com")); // non-video domain
        assert!(!is_video_url("")); // empty string
    }

    #[test]
    fn test_split_base64_url() {
        let src = "data:image/gif;base64,R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==";
        let (image_type, image_data) = split_base64_url(src).unwrap();
        assert_eq!(image_type, "image/gif");
        assert_eq!(
            image_data,
            "R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw=="
        );

        // Test empty base64 data
        let src = "data:image/gif;base64,";
        assert!(split_base64_url(src).is_none());

        // Test invalid data URL format
        let src = "invalid:image/gif;base64,data";
        assert!(split_base64_url(src).is_none());

        // Test missing base64 marker
        let src = "data:image/gif,R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==";
        assert!(split_base64_url(src).is_none());
    }
}
