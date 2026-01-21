mod common;

use common::test_readability;

#[test]
fn test_001() {
    test_readability("test-pages/ok/001/");
}

#[test]
fn test_002() {
    test_readability("test-pages/ok/002/");
}

#[test]
fn test_003() {
    test_readability("test-pages/ok/003-metadata-preferred/");
}

#[test]
fn test_004() {
    test_readability("test-pages/ok/004-metadata-space-separated-properties/");
}

#[test]
fn test_005() {
    test_readability("test-pages/ok/005-unescape-html-entities/");
}

#[test]
fn test_aclu() {
    test_readability("test-pages/ok/aclu/");
}

#[test]
fn test_aktualne() {
    test_readability("test-pages/ok/aktualne/");
}

#[test]
fn test_archive_of_our_own() {
    test_readability("test-pages/ok/archive-of-our-own/");
}

#[test]
fn test_ars_1() {
    test_readability("test-pages/ok/ars-1/");
}

#[test]
fn test_base_url() {
    test_readability("test-pages/ok/base-url/");
}

#[test]
fn test_base_url_base_element_relative() {
    test_readability("test-pages/ok/base-url-base-element-relative/");
}

#[test]
fn test_breitbart() {
    test_readability("test-pages/ok/breitbart/");
}

#[test]
fn test_clean_links() {
    test_readability("test-pages/ok/clean-links/");
}

#[test]
fn test_cnn() {
    test_readability("test-pages/ok/cnn/");
}

#[test]
fn test_ehow_1() {
    test_readability("test-pages/ok/ehow-1/");
}

#[test]
fn test_js_link_replacement() {
    test_readability("test-pages/ok/js-link-replacement/");
}

#[test]
fn test_keep_tabular_data() {
    test_readability("test-pages/ok/keep-tabular-data/");
}

#[test]
fn test_medicalnewstoday() {
    test_readability("test-pages/ok/medicalnewstoday/");
}

#[test]
fn test_medium_3() {
    test_readability("test-pages/ok/medium-3/");
}

#[test]
fn test_qq() {
    test_readability("test-pages/ok/qq/");
}

#[test]
fn test_replace_brs() {
    test_readability("test-pages/ok/replace-brs/");
}

#[test]
fn test_social_buttons() {
    test_readability("test-pages/ok/social-buttons/");
}

#[test]
fn test_tmz_1() {
    test_readability("test-pages/ok/tmz-1/");
}

#[test]
fn test_wikia() {
    test_readability("test-pages/ok/wikia/");
}

#[test]
fn test_wikipedia() {
    test_readability("test-pages/ok/wikipedia/");
}

#[test]
fn test_gmw() {
    test_readability("test-pages/ok/gmw/");
}

#[test]
fn test_videos_1() {
    test_readability("test-pages/ok/videos-1/")
}

#[test]
fn test_v8_blog() {
    test_readability("test-pages/ok/v8-blog/")
}

#[test]
fn test_lwn_1() {
    test_readability("test-pages/ok/lwn-1/");
}

#[test]
fn test_ietf_1() {
    test_readability("test-pages/ok/ietf-1/")
}

#[test]
fn test_toc_missing() {
    test_readability("test-pages/ok/toc-missing/")
}

#[test]
fn test_table_style_attributes() {
    test_readability("test-pages/ok/table-style-attributes/")
}

#[test]
fn test_dev418() {
    test_readability("test-pages/ok/dev418/");
}

#[test]
fn test_citylab_1() {
    test_readability("test-pages/ok/citylab-1/");
}

#[test]
fn test_lemonde_1() {
    test_readability("test-pages/readability/lemonde-1/");
}

#[test]
fn test_hukumusume() {
    test_readability("test-pages/ok/hukumusume/");
}

#[test]
fn test_engadget() {
    // this seems ok
    test_readability("test-pages/ok/engadget/");
}

#[test]
fn test_la_nacion() {
    // this seems ok
    test_readability("test-pages/ok/la-nacion/");
}

#[test]
fn test_wikipedia_3() {
    // this seems ok
    test_readability("test-pages/ok/wikipedia-3/");
}

#[test]
fn test_wikipedia_2() {
    // this seems ok
    test_readability("test-pages/ok/wikipedia-2/");
}

#[test]
fn arstechnica() {
    // this seems ok
    test_readability("test-pages/alt/arstechnica/");
}

#[test]
fn test_wapo_1() {
    // this seems ok
    test_readability("test-pages/readability/wapo-1/");
}