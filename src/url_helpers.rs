fn delimiter(s: &str) -> &str {
    let mut count = 0;
    for (i, c) in s.char_indices() {
        if c == ':' || c == '/' {
            count = i + c.len_utf8();
            if count >= 3 {
                break;
            }
        } else {
            break;
        }
    }
    &s[..count]
}

/// Checks whether the string is an absolute URL.
///
/// - If `strict` is `true`, requires the scheme to be followed by `://`
///   (e.g. `http://`, `https://`, `ftp://`).
/// - If `strict` is `false`, accepts any scheme followed by `:`
///   (e.g. `mailto:`, `data:`, `blob:`).
pub(crate) fn is_absolute_url(s: &str, strict: bool) -> bool {
    let s = s.trim();
    let delim = if strict { "://" } else { ":" };
    if let Some(pos) = s.find(delim) {
        let scheme = &s[..pos];
        let mut chars = scheme.chars();
        if let Some(first) = chars.next() {
            if first.is_ascii_alphabetic()
                && chars.all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
            {
                if strict {
                    return s.len() > pos + delim.len() && !s.ends_with(delim);
                }
                return s.len() > pos + delimiter(&s[pos..]).len();
            }
        }
    }
    false
}

pub(crate) fn to_absolute_url(raw_url: &str, base_uri: &str) -> String {
    let u = if raw_url.starts_with("file://") {
        raw_url.replace("|/", ":/")
    } else {
        raw_url.to_string()
    };
    url_join(base_uri, &u)
}

pub(crate) fn url_join(base: &str, relative: &str) -> String {
    let rel = relative.trim();
    if rel.is_empty() {
        return base.to_string();
    }
    if is_absolute_url(rel, false) {
        return rel.to_string();
    }

    // 1. Find the scheme of the base URL
    let Some(scheme_end) = base.find(':') else {
        return rel.to_string();
    };
    let scheme = &base[..scheme_end];

    // 2. Handle relative URLs starting with "//": //example.com/path
    if rel.starts_with("//") {
        return format!("{scheme}:{rel}");
    }

    // 3. Find the end of origin (scheme://authority)
    let origin_end = base
        .find("://")
        .map_or(scheme_end + 1,|pos| {
            base[pos + 3..]
                .find('/')
                .map_or(base.len(),|p| p + pos + 3)
        });
    let origin = &base[..origin_end];

    // 4. Links, starting with root: /path/to/file
    if rel.starts_with('/') {
        return format!("{origin}{rel}");
    }

    // 5. Split path from query and fragment in base URL
    let base_path_full = &base[origin_end..];
    let query_start = base_path_full
        .find(['?', '#'])
        .unwrap_or(base_path_full.len());
    let base_path = &base_path_full[..query_start];

    // 6. Links with query/fragment: ?id=123 or #anchor
    if rel.starts_with(['?', '#']) {
        return format!("{origin}{base_path}{rel}");
    }
    // 7. The most complex case: relative paths (cat.jpg, ../img/dog.jpg)

    // Building a new path, handling ".." и "."
    let mut path_segments: Vec<&str> = base_path
        .rsplit_once('/')
        .map(|(dir, _)| dir.split('/').filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();

    for segment in rel.split('/') {
        match segment {
            "." => {},
            ".." => {
                path_segments.pop();
            }
            _ => path_segments.push(segment),
        }
    }

    let final_path = path_segments.join("/");
    format!("{origin}/{final_path}")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_urls() {
        assert!(is_absolute_url("http://example.com", true));
        assert!(is_absolute_url("https://example.com", true));
        assert!(is_absolute_url("git+ssh://example.com", true));
        assert!(is_absolute_url("a-b://x", true));
        assert!(is_absolute_url("x.y://zzz", true));
        assert!(is_absolute_url("mailto:foo@bar.com", false));
    }

    #[test]
    fn test_invalid_urls() {
        assert!(!is_absolute_url("://example.com", true));
        assert!(!is_absolute_url("example.com", true));
        assert!(!is_absolute_url("example.com", false));
        assert!(!is_absolute_url("1http://example.com", true));
        assert!(!is_absolute_url("-abc://test", true));
        assert!(!is_absolute_url("   ", true));
        assert!(!is_absolute_url("   ", false));
        assert!(!is_absolute_url("mailto:foo@bar.com", true));
        assert!(!is_absolute_url("https://", true));
        assert!(!is_absolute_url("http://", false));
        assert!(!is_absolute_url("mailto:", false));
    }

    #[test]
    fn test_url_join() {
        let tests = [
            // (base, relative, expected)
            ("http://example.com/path/page.html", "image.jpg", "http://example.com/path/image.jpg"),
            ("http://example.com/path/page.html", "/image.jpg", "http://example.com/image.jpg"),

            ("http://fakehost/test", 
            "/syndication/reuse-permision-form?url=http://www.independent.co.uk/news/business/news/seven-secrets-that-hotel-owners-dont-want-you-to-know-10506160.html", 
            "http://fakehost/syndication/reuse-permision-form?url=http://www.independent.co.uk/news/business/news/seven-secrets-that-hotel-owners-dont-want-you-to-know-10506160.html"),

            ("http://fakehost/test",
            "//img-aws.ehowcdn.com/cdn-write.demandstudios.com/upload/image/2F/86/0496F61C862F.jpg", 
            "http://img-aws.ehowcdn.com/cdn-write.demandstudios.com/upload/image/2F/86/0496F61C862F.jpg"),

            ("http://example.com/path/page.html", "blob:http://www.independent.co.uk/112e1cb2-b0b1-e146-be22-fc6d052f7ddd", 
            "blob:http://www.independent.co.uk/112e1cb2-b0b1-e146-be22-fc6d052f7ddd"),
            ("http://fakehost/test/", "./W020170310313653868929.jpg", "http://fakehost/test/W020170310313653868929.jpg"),
            ("http://fakehost/test/", "../../../../366/logo_bana/corner_2.gif", "http://fakehost/366/logo_bana/corner_2.gif"),
            ("http://example.com/path/page.html", "foo//bar", "http://example.com/path/foo//bar"),
            ("http://example.com/", "data:text/plain,hello", "data:text/plain,hello"),
            ("http://example.com/", "../foo", "http://example.com/foo"),
            ("http://example.com/path/page.html", "", "http://example.com/path/page.html"),
            ("http://example.com/path/page.html", "https://other.com/file", "https://other.com/file"),
            ("http://example.com/path/page.html", "?id=123", "http://example.com/path/page.html?id=123"),
            // no punycode conversion
            ("https://café.com", "menu.html", "https://café.com/menu.html"),
        ];

        for (base, relative, expected) in tests {
            let result = url_join(base, relative);
            assert_eq!(
                result, expected,
                "Failed for base: {}, relative: {}",
                base, relative
            );
        }
    }
}
