use dom_smoothie::{Article, CandidateSelectMode, Config, Readability, TextMode};

#[test]
fn test_formatted_text() {
    let html = include_str!("../test-pages/alt/arstechnica/source.html");

    // for more options check the documentation
    let cfg = Config {
        candidate_select_mode: CandidateSelectMode::DomSmoothie,
        text_mode: TextMode::Formatted,
        ..Default::default()
    };
    let mut readability = Readability::new(html, None, Some(cfg)).unwrap();

    let article: Article = readability.parse().unwrap();
    let expected_text = include_str!("../test-pages/alt/arstechnica/expected.txt");
    let article_text = article.text_content.as_ref();
    assert_eq!(article_text, expected_text.trim())
}
