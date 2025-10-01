mod common;

use common::test_favicon;

#[test]
fn test_favicon_aclu() {
    test_favicon(
        "./test-pages/ok/aclu",
        Some("http://fakehost/test/"),
        Some("http://fakehost/sites/all/themes/custom/aclu/favicons/favicon-32x32.png?v=1")
    );
}


#[test]
fn test_favicon_aktualne() {
    test_favicon(
        "./test-pages/ok/aktualne",
        Some("http://fakehost/test/"),
        Some("http://asset.stdout.cz/fe/aktualne/img/android-chrome-192x192.png")
    );
}

#[test]
fn test_favicon_breitbart() {
    // Expecting None, because, this document has a broken markup (head section).
    test_favicon(
        "./test-pages/ok/breitbart",
        Some("http://fakehost/test/"),
        None
    );
}


#[test]
fn test_favicon_engadget() {
    test_favicon(
        "./test-pages/ok/engadget",
        Some("http://fakehost/test/"),
        Some("https://s.blogsmithmedia.com/www.engadget.com/assets-h159e8c9b49d08fd74b1f658dac6e12df/images/favicon-160x160.png?h=1638b0a8bbe7effa8f85c3ecabb63620")
    );
}

#[test]
fn test_favicon_folha() {
    test_favicon(
        "./test-pages/readability/folha",
        Some("http://fakehost/test/"),
        Some("http://f.i.uol.com.br/hunting/folha/1/common/icons/favicon-192.png")
    );
}


#[test]
fn test_favicon_gitlab_blog() {
    test_favicon(
        "./test-pages/readability/gitlab-blog",
        Some("http://fakehost/test/"),
        Some("http://fakehost/blog/nuxt-images/ico/favicon-192x192.png?cache=2022041")
    );
}

#[test]
fn test_favicon_gitlab_nytimes_2() {
    test_favicon(
        "./test-pages/readability/nytimes-2",
        Some("http://fakehost/test/"),
        Some("https://static01.nyt.com/favicon.ico")
    );
}


#[test]
fn test_favicon_schema_org_context() {
    test_favicon(
        "./test-pages/readability/schema-org-context-object",
        Some("http://fakehost/test/"),
        Some("https://nodeassets.nbcnews.com/cdnassets/projects/ramen/favicon/nbcnews/all-other-sizes-PNG.ico/favicon-96x96.png")
    );
}

#[test]
fn test_favicon_topicseed_1() {
    test_favicon(
        "./test-pages/readability/topicseed-1",
        Some("http://fakehost/test/"),
        Some("http://fakehost/icons-5f7390f8df97283d5b517dead9b4a9d5/favicon-32x32.png")
    );
}


#[test]
fn test_favicon_yahoo_1() {
    test_favicon(
        "./test-pages/readability/yahoo-1",
        Some("http://fakehost/test/"),
        Some("https://s.yimg.com/os/mit/media/p/common/images/favicon_new-7483e38.svg")
    );
}