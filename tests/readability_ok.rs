mod common;

use common::test_readability;

#[test]
fn test_001() {
    test_readability("test-pages/ok/001/", Some("http://fakehost/test/"));
}

#[test]
fn test_002() {
    test_readability("test-pages/ok/002/", Some("http://fakehost/test/"));
}

#[test]
fn test_003() {
    test_readability(
        "test-pages/ok/003-metadata-preferred/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_004() {
    test_readability(
        "test-pages/ok/004-metadata-space-separated-properties/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_005() {
    test_readability(
        "test-pages/ok/005-unescape-html-entities/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_aclu() {
    test_readability("test-pages/ok/aclu/", Some("http://fakehost/test/"));
}

#[test]
fn test_aktualne() {
    test_readability("test-pages/ok/aktualne/", Some("http://fakehost/test/"));
}

#[test]
fn test_archive_of_our_own() {
    test_readability(
        "test-pages/ok/archive-of-our-own/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_ars_1() {
    test_readability("test-pages/ok/ars-1/", Some("http://fakehost/test/"));
}

#[test]
fn test_base_url() {
    test_readability("test-pages/ok/base-url/", Some("http://fakehost/test/"));
}

#[test]
fn test_base_url_base_element_relative() {
    test_readability(
        "test-pages/ok/base-url-base-element-relative/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_breitbart() {
    test_readability("test-pages/ok/breitbart/", Some("http://fakehost/test/"));
}

#[test]
fn test_clean_links() {
    test_readability("test-pages/ok/clean-links/", Some("http://fakehost/test/"));
}

#[test]
fn test_cnn() {
    test_readability("test-pages/ok/cnn/", Some("http://fakehost/test/"));
}

#[test]
fn test_ehow_1() {
    test_readability("test-pages/ok/ehow-1/", Some("http://fakehost/test/"));
}

#[test]
fn test_js_link_replacement() {
    test_readability(
        "test-pages/ok/js-link-replacement/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_keep_tabular_data() {
    test_readability(
        "test-pages/ok/keep-tabular-data/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_medicalnewstoday() {
    test_readability(
        "test-pages/ok/medicalnewstoday/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_medium_3() {
    test_readability("test-pages/ok/medium-3/", Some("http://fakehost/test/"));
}

#[test]
fn test_qq() {
    test_readability("test-pages/ok/qq/", Some("http://fakehost/test/"));
}

#[test]
fn test_replace_brs() {
    test_readability("test-pages/ok/replace-brs/", Some("http://fakehost/test/"));
}

#[test]
fn test_social_buttons() {
    test_readability(
        "test-pages/ok/social-buttons/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_tmz_1() {
    test_readability("test-pages/ok/tmz-1/", Some("http://fakehost/test/"));
}

#[test]
fn test_wikia() {
    test_readability("test-pages/ok/wikia/", Some("http://fakehost/test/"));
}

#[test]
fn test_wikipedia() {
    test_readability("test-pages/ok/wikipedia/", Some("http://fakehost/test/"));
}

#[test]
fn test_gmw() {
    test_readability("test-pages/ok/gmw/", Some("http://fakehost/test/"));
}

#[test]
fn test_videos_1() {
    test_readability("test-pages/ok/videos-1/", Some("http://fakehost/test/"))
}

#[test]
fn test_v8_blog() {
    test_readability("test-pages/ok/v8-blog/", Some("http://fakehost/test/"))
}

#[test]
fn test_lwn_1() {
    test_readability("test-pages/ok/lwn-1/", Some("http://fakehost/test/"));
}

#[test]
fn test_ietf_1() {
    test_readability("test-pages/ok/ietf-1/", Some("http://fakehost/test/"))
}

#[test]
fn test_toc_missing() {
    test_readability("test-pages/ok/toc-missing/", Some("http://fakehost/test/"))
}

#[test]
fn test_table_style_attributes() {
    test_readability(
        "test-pages/ok/table-style-attributes/",
        Some("http://fakehost/test/"),
    )
}

#[test]
fn test_dev418() {
    test_readability("test-pages/ok/dev418/", Some("http://fakehost/test/"));
}

#[test]
fn test_citylab_1() {
    test_readability("test-pages/ok/citylab-1/", Some("http://fakehost/test/"));
}

#[test]
fn test_lemonde_1() {
    test_readability(
        "test-pages/readability/lemonde-1/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_hukumusume() {
    test_readability("test-pages/ok/hukumusume/", Some("http://fakehost/test/"));
}

#[test]
fn test_engadget() {
    // this seems ok
    test_readability("test-pages/ok/engadget/", Some("http://fakehost/test/"));
}

#[test]
fn test_la_nacion() {
    // this seems ok
    test_readability("test-pages/ok/la-nacion/", Some("http://fakehost/test/"));
}

#[test]
fn test_wikipedia_3() {
    // this seems ok
    test_readability("test-pages/ok/wikipedia-3/", Some("http://fakehost/test/"));
}

#[test]
fn test_wikipedia_2() {
    // this seems ok
    test_readability("test-pages/ok/wikipedia-2/", Some("http://fakehost/test/"));
}

#[test]
fn arstechnica() {
    // this seems ok
    test_readability("test-pages/ok/arstechnica/", Some("http://fakehost/test/"));
}
