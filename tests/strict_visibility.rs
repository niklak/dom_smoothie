use dom_smoothie::{Config, Readability};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

static PARAGRAPH: &str = "<p>Bread is a staple food prepared from a dough of flour and water, usually by baking. \
    Throughout recorded history and around the world, it has been an important part of many cultures' diet. \
    It is one of the oldest human-made foods, and it plays an essential role in religious rituals and secular culture.</p>";

/// Each snippet holds its needle, which leaks into the text without the flag.
static HIDDEN: &[(&str, &str)] = &[
    (
        "H_display_comment",
        r#"<span style="display:/**/none">H_display_comment</span>"#,
    ),
    (
        "H_collapse",
        r#"<span style="visibility:collapse">H_collapse</span>"#,
    ),
    (
        "H_aria_upper",
        r#"<span aria-hidden="TRUE">H_aria_upper</span>"#,
    ),
    (
        "H_font_size",
        r#"<span style="font-size:0">H_font <b>H_font_size</b></span>"#,
    ),
    (
        "H_offscreen",
        r#"<span style="position:absolute;left:-9999px">H_offscreen</span>"#,
    ),
    (
        "H_clip",
        r#"<span style="position:absolute;clip:rect(1px, 1px, 1px, 1px)">H_clip</span>"#,
    ),
    (
        "H_clip_path",
        r#"<span style="position:absolute;clip-path:inset(50%)">H_clip_path</span>"#,
    ),
    (
        "H_zero_size",
        r#"<span style="display:inline-block;width:0;height:0;overflow:hidden">H_zero_size</span>"#,
    ),
    (
        "H_content_visibility",
        r#"<span style="display:inline-block;visibility:visible;content-visibility:hidden">H_content_visibility</span>"#,
    ),
    ("H_sr_only", r#"<span class="sr-only">H_sr_only</span>"#),
    (
        "H_screen_reader_text",
        r#"<span class="screen-reader-text">H_screen_reader_text</span>"#,
    ),
    ("H_video", r#"<video>H_video<source src="v.mp4"></video>"#),
    (
        "H_datalist",
        "<datalist><option>H_datalist</option></datalist>",
    ),
    ("H_rp", "<ruby>base<rp>H_rp</rp><rt>rt</rt></ruby>"),
    (
        "H_svg_title",
        "<svg><title>H_svg_title</title><text>svg</text></svg>",
    ),
    (
        "H_annotation",
        "<math><mi>x</mi><annotation>H_annotation</annotation></math>",
    ),
    ("\u{E0049}", "tag\u{E0049}chars"),
    ("\u{FEFF}", "\u{FEFF}bom"),
    ("\u{E0041}", "<code>code\u{E0041}</code>"),
    (
        "H_rating_twin",
        r#"<span aria-hidden="true">x</span><span class="sr-only">H_rating_twin</span>"#,
    ),
];

static VISIBLE: &[&str] = &[
    r#"<span style="color:#333;position:relative;left:-10px">V_normal</span>"#,
    r#"<span style="opacity:0;transform:translate3d(0,40px,0)">V_webflow</span>"#,
    r#"<span style="background-clip:text;color:transparent">V_gradient</span>"#,
    r#"<span style="clip:rect(0 0 0 0)">V_clip_static</span>"#,
    r#"<span style="height:0;padding-bottom:56%;overflow:hidden">V_aspect_box</span>"#,
    r#"<span style="font-size:0"><span style="font-size:16px">V_font_size_gap</span></span>"#,
    r#"<span class="hidden hide d-none invisible">V_toggle_classes</span>"#,
    r#"<span class="sr-only focus:not-sr-only">V_sr_focus</span>"#,
    r#"<span aria-hidden="true" class="fallback-image">V_fallback_image</span>"#,
    r#"<style>.x{display:none}</style><span class="x">V_sheet</span>"#,
    "<span>V_rtl_\u{2067}isolate\u{2069}_\u{00AD}_\u{200D}_\u{200B}</span>",
    "<span>V_flag_\u{1F3F4}\u{E0067}\u{E0062}\u{E0073}\u{E0063}\u{E0074}\u{E007F}</span>",
    r#"<span style="position:absolute;top:-100%">V_tooltip</span>"#,
    r#"<span style="position:absolute;clip-path:inset(50% 0 0 0)">V_half_clipped</span>"#,
    r#"<span style="transform:scale(0);transition:transform .3s">V_scale_start</span>"#,
    r#"<span style="visibility:collapse;visibility:visible">V_override</span>"#,
];

fn text_content(body_attrs: &str, snippets: &str, strict_visibility: bool) -> String {
    let html = format!(
        "<html><body {body_attrs}><article><h1>Bread</h1>{PARAGRAPH}<p>{snippets}</p>{PARAGRAPH}</article></body></html>"
    );
    let cfg = Config {
        strict_visibility,
        ..Default::default()
    };
    let mut ra = Readability::new(html, None, Some(cfg)).unwrap();
    ra.parse().unwrap().text_content.to_string()
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_strict_visibility_removes_hidden_text() {
    let snippets: String = HIDDEN
        .iter()
        .map(|(_, s)| *s)
        .chain(VISIBLE.iter().copied())
        .collect();
    let off = text_content("", &snippets, false);
    let on = text_content("", &snippets, true);
    for (needle, _) in HIDDEN {
        assert!(
            off.contains(needle),
            "{needle:?} is already hidden without the flag"
        );
        assert!(!on.contains(needle), "{needle:?} leaked");
    }
    for snippet in VISIBLE {
        let start = snippet.find("V_").unwrap();
        let token = &snippet[start..start + snippet[start..].find('<').unwrap()];
        assert!(off.contains(token) && on.contains(token), "{token:?} lost");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_strict_visibility_keeps_body() {
    for body_attrs in [
        r#"style="opacity:0""#,
        r#"class="sr-only""#,
        r#"style="display:none""#,
    ] {
        assert!(
            text_content(body_attrs, "", true).contains("staple food"),
            "{body_attrs}"
        );
    }
}
