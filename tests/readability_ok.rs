#[macro_use]
mod common;

use common::{test_readability, TestData};

#[test]
fn test_001() {
    test_readability(test_data!("test-pages/ok/001/", "expected.html"));
}

#[test]
fn test_002() {
    test_readability(test_data!("test-pages/ok/002/", "expected.html"));
}

#[test]
fn test_003() {
    test_readability(test_data!(
        "test-pages/ok/003-metadata-preferred/",
        "expected.html"
    ));
}

#[test]
fn test_004() {
    test_readability(test_data!(
        "test-pages/ok/004-metadata-space-separated-properties/",
        "expected.html"
    ));
}

#[test]
fn test_005() {
    test_readability(test_data!(
        "test-pages/ok/005-unescape-html-entities/",
        "expected.html"
    ));
}

#[test]
fn test_aclu() {
    test_readability(test_data!("test-pages/ok/aclu/", "expected.html"));
}

#[test]
fn test_aktualne() {
    test_readability(test_data!("test-pages/ok/aktualne/", "expected.html"));
}

#[test]
fn test_archive_of_our_own() {
    test_readability(test_data!(
        "test-pages/ok/archive-of-our-own/",
        "expected.html"
    ));
}

#[test]
fn test_ars_1() {
    test_readability(test_data!("test-pages/ok/ars-1/", "expected.html"));
}

#[test]
fn test_base_url() {
    test_readability(test_data!("test-pages/ok/base-url/", "expected.html"));
}

#[test]
fn test_base_url_base_element_relative() {
    test_readability(test_data!(
        "test-pages/ok/base-url-base-element-relative/",
        "expected.html"
    ));
}

#[test]
fn test_breitbart() {
    test_readability(test_data!("test-pages/ok/breitbart/", "expected.html"));
}

#[test]
fn test_clean_links() {
    test_readability(test_data!("test-pages/ok/clean-links/", "expected.html"));
}

#[test]
fn test_cnn() {
    test_readability(test_data!("test-pages/ok/cnn/", "expected.html"));
}

#[test]
fn test_ehow_1() {
    test_readability(test_data!("test-pages/ok/ehow-1/", "expected.html"));
}

#[test]
fn test_js_link_replacement() {
    test_readability(test_data!(
        "test-pages/ok/js-link-replacement/",
        "expected.html"
    ));
}

#[test]
fn test_keep_tabular_data() {
    test_readability(test_data!(
        "test-pages/ok/keep-tabular-data/",
        "expected.html"
    ));
}

#[test]
fn test_medicalnewstoday() {
    test_readability(test_data!(
        "test-pages/ok/medicalnewstoday/",
        "expected.html"
    ));
}

#[test]
fn test_medium_3() {
    test_readability(test_data!("test-pages/ok/medium-3/", "expected.html"));
}

#[test]
fn test_qq() {
    test_readability(test_data!("test-pages/ok/qq/", "expected.html"));
}

#[test]
fn test_replace_brs() {
    test_readability(test_data!("test-pages/ok/replace-brs/", "expected.html"));
}

#[test]
fn test_social_buttons() {
    test_readability(test_data!("test-pages/ok/social-buttons/", "expected.html"));
}

#[test]
fn test_tmz_1() {
    test_readability(test_data!("test-pages/ok/tmz-1/", "expected.html"));
}

#[test]
fn test_wikia() {
    test_readability(test_data!("test-pages/ok/wikia/", "expected.html"));
}

#[test]
fn test_wikipedia() {
    test_readability(test_data!("test-pages/ok/wikipedia/", "expected.html"));
}

#[test]
fn test_gmw() {
    test_readability(test_data!("test-pages/ok/gmw/", "expected.html"));
}

#[test]
fn test_videos_1() {
    test_readability(test_data!("test-pages/ok/videos-1/", "expected.html"));
}

#[test]
fn test_v8_blog() {
    test_readability(test_data!("test-pages/ok/v8-blog/", "expected.html"));
}

#[test]
fn test_lwn_1() {
    test_readability(test_data!("test-pages/ok/lwn-1/", "expected.html"));
}

#[test]
fn test_ietf_1() {
    test_readability(test_data!("test-pages/ok/ietf-1/", "expected.html"));
}

#[test]
fn test_toc_missing() {
    test_readability(test_data!("test-pages/ok/toc-missing/", "expected.html"));
}

#[test]
fn test_table_style_attributes() {
    test_readability(test_data!(
        "test-pages/ok/table-style-attributes/",
        "expected.html"
    ));
}

#[test]
fn test_dev418() {
    test_readability(test_data!("test-pages/ok/dev418/", "expected.html"));
}

#[test]
fn test_citylab_1() {
    test_readability(test_data!("test-pages/ok/citylab-1/", "expected.html"));
}

#[test]
fn test_lemonde_1() {
    test_readability(test_data!(
        "test-pages/readability/lemonde-1/",
        "expected.html"
    ));
}

#[test]
fn test_hukumusume() {
    test_readability(test_data!("test-pages/ok/hukumusume/", "expected.html"));
}

#[test]
fn test_engadget() {
    // this seems ok
    test_readability(test_data!("test-pages/ok/engadget/", "expected.html"));
}

#[test]
fn test_la_nacion() {
    // this seems ok
    test_readability(test_data!("test-pages/ok/la-nacion/", "expected.html"));
}

#[test]
fn test_wikipedia_3() {
    // this seems ok
    test_readability(test_data!("test-pages/ok/wikipedia-3/", "expected.html"));
}

#[test]
fn test_wikipedia_2() {
    // this seems ok
    test_readability(test_data!("test-pages/ok/wikipedia-2/", "expected.html"));
}

#[test]
fn arstechnica() {
    // this seems ok
    test_readability(test_data!("test-pages/alt/arstechnica/", "expected.html"));
}
