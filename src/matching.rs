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
    let trimmed = content.trim_start();
    if let Some(rest) = trimmed.strip_prefix("<![CDATA[") {
        if let Some(pos) = rest.rfind("]]>") {
            return &rest[..pos];
        }
        return rest;
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

#[cfg(test)]
mod tests {

    use super::*;

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
}
