//! Articles a page splits into parts of one kind, with figures between them.

use dom_smoothie::{Config, Readability, dom_query::Document};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

fn prose(n: usize) -> String {
    format!(
        "<p>Paragraph {n} of the story goes on, as stories do, about networks, routers and the \
         packets between them, which, when a link fails, must find another way through, and the \
         researchers, after years of work, showed when that is possible, and when it is not.</p>"
    )
}

fn part(ps: std::ops::Range<usize>) -> String {
    let ps: String = ps.map(prose).collect();
    format!(
        r#"<section class="outer"><div class="flex-auto"><div class="post__content__section"><div class="post__content">{ps}</div></div></div></section>"#
    )
}

/// A figure with a short caption, its image in a wrapper with a negative class (`media`).
fn figure(src: &str) -> String {
    format!(
        r#"<figure class="leading-0 w-full"><div class="acf-media"><div class="relative"><img src="https://example.org/{src}.webp"></div></div><figcaption><div class="attribution"><p>Mark Belan</p></div></figcaption></figure>"#
    )
}

fn parse(html: &str) -> Document {
    let cfg = Config {
        strict_visibility: true,
        ..Default::default()
    };
    let mut r = Readability::new(html, None, Some(cfg)).unwrap();
    Document::from(r.parse().unwrap().content.to_string())
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_figures_between_parts_of_an_article_are_kept() {
    let html = format!(
        r#"<html><body><main><div class="post"><div class="article">
        <div class="acf-content scale1 mt2"><h2 class="screen-reader-text">Introduction</h2><div class="post__wrapper"><div class="mha"><div class="post__content relative">{}</div></div></div></div>
        {}
        <div class="acf-content scale1 mt2">{}</div>
        {}
        <div class="acf-content scale1 mt2">{}</div>
        </div></div></main></body></html>"#,
        part(0..8),
        figure("graph"),
        part(8..12),
        figure("minors"),
        part(12..14),
    );
    let doc = parse(&html);
    let text = doc.text();
    for n in 0..14 {
        assert!(
            text.contains(&format!("Paragraph {n} ")),
            "paragraph {n} missing"
        );
    }
    let srcs: Vec<_> = doc
        .select("figure img")
        .iter()
        .map(|i| i.attr_or("src", "").to_string())
        .collect();
    assert_eq!(
        srcs,
        [
            "https://example.org/graph.webp",
            "https://example.org/minors.webp"
        ]
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_figure_after_the_last_part_is_not_joined() {
    let html = format!(
        r#"<html><body><div class="article">
        <div class="acf-content">{}</div>{}<div class="acf-content">{}</div>{}
        </div></body></html>"#,
        part(0..6),
        figure("between"),
        part(6..10),
        figure("promo"),
    );
    let doc = parse(&html);
    assert!(doc.select(r#"img[src$="between.webp"]"#).exists());
    assert!(!doc.select(r#"img[src$="promo.webp"]"#).exists());
}

/// Three candidates as good as the top one, one of them the article that holds it: the article,
/// not the page around it, is the common ancestor.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_alternative_candidate_that_holds_the_top_one_is_the_common_ancestor() {
    let caption = |n: usize| {
        format!(
            r#"<figure><img src="https://example.org/{n}.webp"><figcaption><p>A long caption, with a comma, and another, and more, about figure {n}, its maker, its date, and its size, in inches.</p></figcaption></figure>"#
        )
    };
    let figures: String = (0..4).map(caption).collect();
    let html = format!(
        r#"<html><body><main><div class="post">
        <div class="article">
          <div class="acf-content scale1"><div class="post__wrapper"><div class="mha"><div class="post__content relative">{}</div></div></div></div>
          {figures}
          <div class="acf-content scale1 mt2">{}</div>
        </div>
        <section class="newsletter"><p>Get highlights of the most important news delivered to your email inbox, every week, for free.</p></section>
        </div></main></body></html>"#,
        part(0..9),
        part(9..18),
    );
    let doc = parse(&html);
    let text = doc.text();
    assert!(text.contains("Paragraph 0 ") && text.contains("Paragraph 17 "));
    assert!(!text.contains("Get highlights"), "{text}");
}
