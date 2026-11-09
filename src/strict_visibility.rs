//! Opt-in removal of text that a reader cannot see, beyond the checks Readability.js makes.
//! Only inline styles and attributes are read: `<style>` rules need a real cascade.

use dom_query::{Document, NodeData, NodeId, NodeRef};
use phf::phf_set;
use tendril::StrTendril;

use crate::glob::MINI_FALLBACK_IMG;
use crate::helpers::{next_child_or_sibling, node_name_in};

#[rustfmt::skip]
static SCREEN_READER_CLASSES: phf::Set<&'static str> = phf_set!(
    "sr-only", "sr_only", "visually-hidden", "visuallyhidden", "visually_hidden",
    "screen-reader-text", "screen-reader-only", "screenreader-text", "a11y-hidden",
    "assistive-text", "element-invisible",
);

static FALLBACK_PARENTS: phf::Set<&'static str> =
    phf_set!("video", "audio", "canvas", "meter", "progress");
static MEDIA_SOURCES: phf::Set<&'static str> = phf_set!("source", "track");
static SUBDIVISION_FLAG_TAGS: [&str; 3] = [
    "\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}",
    "\u{E0067}\u{E0062}\u{E0073}\u{E0063}\u{E0074}\u{E007F}",
    "\u{E0067}\u{E0062}\u{E0077}\u{E006C}\u{E0073}\u{E007F}",
];

/// Inherited by descendants: the element and whether its font size is 0-1px.
type Scope = (NodeId, bool);

/// Removes elements with text that is hidden from readers, unrendered fallback content,
/// and invisible characters. `html` and `body` are never removed.
pub(crate) fn remove_hidden_content(doc: &Document) {
    let mut hidden = vec![];
    let mut scopes: Vec<Scope> = vec![];
    let mut next_node = next_child_or_sibling(&doc.root(), false);
    while let Some(node) = next_node {
        let parent = node.parent().map(|p| p.id);
        while scopes.last().is_some_and(|s| Some(s.0) != parent) {
            scopes.pop();
        }
        let (_, mut tiny_font) = scopes.last().copied().unwrap_or((node.id, false));
        let is_root = node.has_name("html") || node.has_name("body");
        let style = node
            .attr("style")
            .map(|s| strip_css_comments(&s.to_ascii_lowercase()))
            .unwrap_or_default();
        let unrendered = is_unrendered(&node);
        if unrendered || (!is_root && hides_subtree(&node, &style)) {
            next_node = next_child_or_sibling(&node, true);
            if unrendered || has_text(&node) {
                hidden.push(node);
            }
            continue;
        }
        if node_name_in(&node, &FALLBACK_PARENTS) {
            for child in node.children() {
                if !node_name_in(&child, &MEDIA_SOURCES) {
                    child.remove_from_parent();
                }
            }
        }
        let font_size = style_value(&style, "font-size");
        if !is_root && !font_size.is_empty() {
            tiny_font = is_tiny_length(font_size);
        }
        scopes.push((node.id, tiny_font));
        for child in node.children().iter().filter(|c| c.is_text()) {
            if tiny_font && child.is_nonempty_text() {
                child.remove_from_parent();
            } else {
                child.update(|n| {
                    if let NodeData::Text { ref mut contents } = n.data {
                        strip_invisible_chars(contents);
                    }
                });
            }
        }
        next_node = next_child_or_sibling(&node, false);
    }
    for node in hidden {
        node.remove_from_parent();
    }
}

fn has_text(node: &NodeRef) -> bool {
    node.descendants_it().any(|n| n.is_nonempty_text())
}

fn hides_subtree(node: &NodeRef, style: &str) -> bool {
    let aria_hidden = node
        .attr("aria-hidden")
        .is_some_and(|a| a.trim().eq_ignore_ascii_case("true"));
    if aria_hidden && !MINI_FALLBACK_IMG.match_node(node) {
        return true;
    }
    if node.class().is_some_and(|c| has_screen_reader_class(&c)) {
        return true;
    }
    let get = |name| style_value(style, name);
    let position = get("position");
    let offscreen = ["left", "top", "right", "bottom", "inset"]
        .iter()
        .any(|n| get(n).split_ascii_whitespace().any(is_far_offset));
    get("display") == "none"
        || matches!(get("visibility"), "hidden" | "collapse")
        || get("content-visibility") == "hidden"
        || is_empty_inset(get("clip-path"))
        || (matches!(position, "absolute" | "fixed" | "relative") && offscreen)
        || (matches!(position, "absolute" | "fixed") && is_zero_clip(get("clip")))
        || (matches!(get("overflow"), "hidden" | "clip")
            && ((is_tiny_length(get("height")) && !has_padding(style, true))
                || (is_tiny_length(get("width")) && !has_padding(style, false))))
}

/// Whether vertical (or horizontal) padding opens a visible strip in a box clipped to zero height (or width).
fn has_padding(style: &str, vertical: bool) -> bool {
    let shorthand: Vec<&str> = style_value(style, "padding")
        .split_ascii_whitespace()
        .collect();
    let (sides, indices): ([&str; 2], &[usize]) = match (vertical, shorthand.len()) {
        (true, 1 | 2) => (["padding-top", "padding-bottom"], &[0]),
        (true, 3 | 4) => (["padding-top", "padding-bottom"], &[0, 2]),
        (true, _) => (["padding-top", "padding-bottom"], &[]),
        (false, 1) => (["padding-left", "padding-right"], &[0]),
        (false, 2 | 3) => (["padding-left", "padding-right"], &[1]),
        (false, 4) => (["padding-left", "padding-right"], &[1, 3]),
        (false, _) => (["padding-left", "padding-right"], &[]),
    };
    let opens = |v: &str| !is_tiny_length(v) && !v.starts_with('-') && parse_length(v).is_some();
    sides
        .iter()
        .map(|side| style_value(style, side))
        .chain(indices.iter().map(|&i| shorthand[i]))
        .any(opens)
}

/// A variant class next to the hiding one (`sr-only focus:not-sr-only`) may reveal it.
fn has_screen_reader_class(class: &str) -> bool {
    let mut hidden = false;
    for token in class.split_ascii_whitespace() {
        if token.contains(':') {
            return false;
        }
        hidden |= SCREEN_READER_CLASSES.contains(token);
    }
    hidden
}

fn is_unrendered(node: &NodeRef) -> bool {
    node.qual_name_ref()
        .is_some_and(|name| match name.local.as_ref() {
            "datalist" | "rp" | "annotation" | "annotation-xml" => true,
            "title" | "desc" | "metadata" => &*name.ns == "http://www.w3.org/2000/svg",
            _ => false,
        })
}

/// Strips BOMs and tag characters, except the tags of the three RGI subdivision flags (England, Scotland, Wales).
fn strip_invisible_chars(text: &mut StrTendril) {
    let is_tag = |c: char| matches!(c, '\u{E0000}'..='\u{E007F}');
    let is_invisible = |c: char| c == '\u{FEFF}' || is_tag(c);
    if !text.chars().any(is_invisible) {
        return;
    }
    let mut cleaned = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if !is_invisible(c) {
            cleaned.push(c);
            continue;
        }
        if !is_tag(c) {
            continue;
        }
        let mut run = String::from(c);
        while let Some(t) = chars.next_if(|t| is_tag(*t)) {
            run.push(t);
        }
        if cleaned.ends_with('\u{1F3F4}') && SUBDIVISION_FLAG_TAGS.contains(&run.as_str()) {
            cleaned.push_str(&run);
        }
    }
    *text = StrTendril::from(cleaned);
}

fn strip_css_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        let Some(end) = rest[start..].find("*/") else {
            return out;
        };
        rest = &rest[start + end + 2..];
    }
    out.push_str(rest);
    out
}

/// Returns the value of the last `name` declaration, or `""`.
fn style_value<'a>(style: &'a str, name: &str) -> &'a str {
    style
        .rsplit(';')
        .filter_map(|decl| decl.split_once(':'))
        .find(|(n, _)| n.trim() == name)
        .map_or("", |(_, v)| {
            let v = v.trim();
            v.strip_suffix("!important").unwrap_or(v).trim_end()
        })
}

fn parse_length(value: &str) -> Option<(f64, &str)> {
    let split = value
        .find(|c: char| !(c.is_ascii_digit() || matches!(c, '-' | '+' | '.')))
        .unwrap_or(value.len());
    Some((value[..split].parse().ok()?, &value[split..]))
}

fn is_tiny_length(value: &str) -> bool {
    parse_length(value)
        .is_some_and(|(n, unit)| (0.0..=1.0).contains(&n) && (n == 0.0 || unit == "px"))
}

/// Percentages are left alone: `top: -100%` is a tooltip just above its parent.
fn is_far_offset(value: &str) -> bool {
    parse_length(value).is_some_and(|(n, unit)| match unit {
        "" | "px" | "pt" => n <= -999.0,
        "em" | "rem" | "vw" | "vh" => n <= -100.0,
        _ => false,
    })
}

fn is_zero_clip(value: &str) -> bool {
    function_args(value, "rect(").is_some_and(|args| {
        let parts: Vec<&str> = args
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|p| !p.is_empty())
            .collect();
        parts.len() == 4 && parts.iter().all(|p| is_tiny_length(p))
    })
}

/// True when the insets leave no area: top and bottom, or left and right, add up to 100%.
fn is_empty_inset(value: &str) -> bool {
    let Some(args) = function_args(value, "inset(") else {
        return false;
    };
    let percent = |v| {
        parse_length(v)
            .filter(|&(n, unit)| unit == "%" || n == 0.0)
            .map(|(n, _)| n)
    };
    let insets: Option<Vec<f64>> = args.split_ascii_whitespace().map(percent).collect();
    let (top, right, bottom, left) = match insets.as_deref() {
        Some(&[a]) => (a, a, a, a),
        Some(&[v, h]) => (v, h, v, h),
        Some(&[t, h, b]) => (t, h, b, h),
        Some(&[t, r, b, l]) => (t, r, b, l),
        _ => return false,
    };
    top + bottom >= 100.0 || left + right >= 100.0
}

fn function_args<'a>(value: &'a str, func: &str) -> Option<&'a str> {
    let inner = &value[value.find(func)? + func.len()..];
    Some(&inner[..inner.find(')')?])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn visible_text(html: &str) -> String {
        let doc = Document::from(html);
        remove_hidden_content(&doc);
        let body = doc.select("body").nodes()[0];
        let texts: Vec<_> = body
            .descendants_it()
            .filter(|n| n.is_text())
            .map(|n| n.text().to_string())
            .collect();
        texts
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn style_hides(style: &str) -> bool {
        let doc = Document::from(format!(r#"<p style="{style}">x</p>"#));
        let p = doc.select("p").nodes()[0];
        hides_subtree(&p, &strip_css_comments(&style.to_ascii_lowercase()))
    }

    #[test]
    fn test_inline_styles_that_hide() {
        for style in [
            "display:none",
            "DISPLAY: NONE",
            "display:/* x */none",
            "display: none !important",
            "visibility: hidden",
            "visibility: collapse",
            "content-visibility: hidden",
            "content-visibility: auto; visibility: hidden",
            "display: block; display: none",
            "position: absolute; clip: rect(0 0 0 0)",
            "position: absolute; clip: rect(1px, 1px, 1px, 1px)",
            "position: absolute; clip-path: inset(50%)",
            "position: absolute; clip-path: inset(0 50%)",
            "position: absolute; clip-path: inset(60% 0 40%)",
            "position: absolute; clip-path: inset(0 30% 0 70%)",
            "position: absolute; left: -9999px",
            "position: absolute; top: -999pt",
            "position: fixed; top: -100vh",
            "position: absolute; left: -100em",
            "position: absolute; inset: 0 auto auto -999",
            "overflow: hidden; width: 1px; height: 1px",
            "overflow: clip; width: 0; height: 0",
            "clip-path: inset(50%)",
            "position: relative; left: -9999px",
            "overflow: hidden; height: 0",
            "overflow: hidden; width: 0",
            "overflow: hidden; width: 1px; height: 2px",
            "overflow: hidden; height: 1px; padding: 0",
            "overflow: hidden; height: 0; padding: 0 10px",
            "overflow: hidden; height: 0; padding: 0 10px 0 10px",
            "overflow: hidden; height: 0; padding: 1px 1px 1px 1px 1px",
            "overflow: hidden; height: 0; padding-left: 10px; padding-top: 1px",
            "overflow: hidden; height: 0; padding-bottom: -5px",
            "overflow: hidden; height: 0; padding-bottom: auto",
            "overflow: hidden; width: 0; padding: 10px 0",
            "overflow: hidden; width: 0; padding: 0 0 10px",
            "overflow: hidden; width: 0; padding: 10px 0 10px 0",
            "overflow: hidden; width: 0; padding: 2px 2px 2px 2px 2px",
        ] {
            assert!(style_hides(style), "{style}");
        }
    }

    #[test]
    fn test_inline_styles_that_do_not_hide() {
        for style in [
            "",
            "display: block",
            "display: none-ish",
            "display: none; display: block",
            "visibility: collapse; visibility: visible",
            "--display: none",
            "content-visibility: visible",
            "opacity: 0",
            "color: transparent",
            "text-indent: -9999px",
            "transform: scale(0)",
            "clip: rect(0 0 0 0)",
            "position: absolute; clip: rect(0 100px 100px 0)",
            "position: absolute; clip: rect(0 0 0)",
            "position: absolute; clip: auto",
            "position: absolute; clip-path: inset(49.9%)",
            "position: absolute; clip-path: inset(50% 0 0 0)",
            "position: absolute; clip-path: inset(0 49% 0 50%)",
            "position: absolute; clip-path: inset(50% 49% 49.9%)",
            "position: absolute; clip-path: inset(50px)",
            "position: absolute; clip-path: inset(50% 50% 50% 50% 50%)",
            "position: absolute; clip-path: circle(0)",
            "position: absolute; left: -998px",
            "position: absolute; top: -99vh",
            "position: absolute; top: -100%",
            "position: absolute; bottom: -9999%",
            "position: fixed; top: -100px",
            "left: -9999px",
            "overflow: hidden",
            "overflow: hidden; height: 0; padding-bottom: 56.25%",
            "overflow: hidden; height: 0; padding-top: 2px",
            "overflow: hidden; height: 0; padding: 10px",
            "overflow: hidden; height: 0; padding: 10px 0",
            "overflow: hidden; height: 0; padding: 0 0 5px",
            "overflow: hidden; height: 0; padding: 0 0 5px 0",
            "overflow: hidden; width: 0; padding: 10px",
            "overflow: hidden; width: 0; padding: 0 10px",
            "overflow: hidden; width: 0; padding: 0 10px 0",
            "overflow: hidden; width: 0; padding: 0 10px 0 0",
            "overflow: hidden; width: 0; padding: 0 0 0 10px",
            "overflow: hidden; width: 0; padding-left: 2px",
            "overflow: hidden; width: 0; padding-right: 2px",
            "position: sticky; left: -9999px",
            "position: relative; left: -10px",
            "overflow: visible; width: 0; height: 0",
            "overflow: hidden; width: -1px; height: -1px",
        ] {
            assert!(!style_hides(style), "{style}");
        }
    }

    #[test]
    fn test_style_value_takes_last_declaration() {
        assert_eq!(
            style_value("position: absolute; position: static", "position"),
            "static"
        );
        assert_eq!(style_value("margin-left: -9999px", "left"), "");
    }

    #[test]
    fn test_unterminated_css_comment() {
        assert_eq!(strip_css_comments("a/* b */c/* d"), "ac");
    }

    #[test]
    fn test_screen_reader_classes() {
        assert!(has_screen_reader_class("note sr-only"));
        assert!(has_screen_reader_class("visually-hidden"));
        assert!(!has_screen_reader_class("sr-only focus:not-sr-only"));
        for class in ["hidden", "d-none", "SR-ONLY", "overflow-hidden"] {
            assert!(!has_screen_reader_class(class), "{class}");
        }
    }

    #[test]
    fn test_attributes() {
        let html = r#"<p aria-hidden="TRUE">a</p><p aria-hidden="false">kept1</p>
            <div aria-hidden="true" class="fallback-image"><p>kept2</p></div>"#;
        assert_eq!(visible_text(html), "kept1 kept2");
    }

    #[test]
    fn test_tiny_font_hides_inherited_text() {
        let html = r#"<p style="font-size:0">gone <b>gone</b></p><p>kept1</p>
            <div style="font-size:1px">gone<span style="font-size:16px">kept2 <i>kept3</i></span></div>
            <p style="font-size:0"><span>gone</span> <span style="font-size:1rem">kept4</span></p>
            <p style="font-size:1.5px">kept5</p><p style="font-size:-1px">kept6</p>"#;
        assert_eq!(visible_text(html), "kept1 kept2 kept3 kept4 kept5 kept6");
    }

    #[test]
    fn test_hidden_subtree_without_text_is_kept() {
        let doc = Document::from(r#"<p style="display:none"> <img src="a.png"> </p>"#);
        remove_hidden_content(&doc);
        assert_eq!(doc.select("img").length(), 1);
    }

    #[test]
    fn test_root_elements_are_kept() {
        let html = r#"<html class="sr-only" style="font-size:0"><body style="display:none;font-size:0"><p>kept</p></body></html>"#;
        assert_eq!(visible_text(html), "kept");
    }

    #[test]
    fn test_unrendered_content() {
        let html = r#"<video>v<source src="a.mp4"></video><meter>m</meter>
            <datalist><option>d</option></datalist><ruby>base<rp>(</rp><rt>rt</rt><rp>)</rp></ruby>
            <svg><title>t</title><desc>d</desc><metadata>m</metadata><text>svg</text></svg>
            <math><mi>x</mi><annotation>a</annotation><annotation-xml>a</annotation-xml></math>"#;
        assert_eq!(visible_text(html), "base rt svg x");
        let doc = Document::from(format!("<head><title>kept</title></head>{html}"));
        remove_hidden_content(&doc);
        assert_eq!(doc.select("source").length(), 1);
        assert_eq!(doc.select("title").text().as_ref(), "kept");
    }

    #[test]
    fn test_invisible_chars() {
        let kept = "\u{200B} \u{00AD}\u{202E}\u{2067} \u{1F468}\u{200D}\u{1F469} \u{0628}\u{200C}\u{062A}\u{200F}";
        let flag = "\u{1F3F4}\u{E0067}\u{E0062}\u{E0073}\u{E0063}\u{E0074}\u{E007F}";
        let html = format!(
            "<p>a\u{E0049}b\u{FEFF}c {kept} {flag} \u{1F3F4}\u{E007F} \u{1F3F4}\u{E0001}\u{E007F} \u{E0067}\u{E007F} \u{1F3F4}\u{E0069}\u{E0067}\u{E006E}\u{E007F}</p>"
        );
        assert_eq!(
            visible_text(&html),
            format!("abc {kept} {flag} \u{1F3F4} \u{1F3F4} \u{1F3F4}")
        );
    }

    #[test]
    fn test_deep_document() {
        for (style, expected) in [
            ("display:none", ""),
            ("font-size:0", ""),
            ("color:red", "x"),
        ] {
            let doc = Document::from("<p>");
            let mut parent = doc.select("p").nodes()[0];
            for _ in 0..10_000 {
                let child = doc.tree.new_element("div");
                child.set_attr("style", style);
                parent.append_child(&child);
                parent = child;
            }
            parent.append_child(&doc.tree.new_text("x"));
            remove_hidden_content(&doc);
            assert_eq!(doc.select("body").text().as_ref(), expected);
        }
    }
}
