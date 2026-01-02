use dom_smoothie::Readability;

#[test]
fn test_skip_body_ancestor() {
    let contents = r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <meta charset="utf-8">
            <title>Some Title</title>
            <link rel="stylesheet" href="style.css">
        </head>
        <body>
            <p><a class="button" href="https://example.com/sign-up"> Sign Up for Live Updates!</a></p>
        </body>
    </html>
        "#;

    let mut ra = Readability::new(contents, None, None).unwrap();
    let res = ra.parse().unwrap();
    let expected: String = r#"<div id="readability-page-1" class="page">
        <p><a href="https://example.com/sign-up"> Sign Up for Live Updates!</a></p>
        </div>"#
        .split_whitespace()
        .collect();
    let got: String = res.content.split_whitespace().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_skip_body_ancestor_fragment() {
    let contents = r#"
    <div>
        <p><a class="button" href="https://example.com/sign-up"> Sign Up for Live Updates!</a></p>
    </div>
    "#;

    let mut ra = Readability::new(contents, None, None).unwrap();
    let res = ra.parse().unwrap();
    let expected: String = r#"<div id="readability-page-1" class="page"><div>
        <p><a href="https://example.com/sign-up"> Sign Up for Live Updates!</a></p>
        </div></div>"#
        .split_whitespace()
        .collect();
    let got: String = res.content.split_whitespace().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_fragments() {
    let contents_0 = r#"
    <body>
        <p><a class="button" href="https://example.com/sign-up"> Sign Up for Live Updates!</a></p>
    </body>
    "#;

    let contents_1 = r#"
        <p><a class="button" href="https://example.com/sign-up"> Sign Up for Live Updates!</a></p>
    "#;

    let cases = vec![contents_0, contents_1];

    for contents in cases {
        let doc = dom_query::Document::fragment(contents);

        let mut ra = Readability::with_document(doc, None, None).unwrap();
        let err = ra.parse().unwrap_err();
        assert!(matches!(err, dom_smoothie::ReadabilityError::GrabFailed));
    }
}
