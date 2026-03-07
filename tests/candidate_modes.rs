use dom_smoothie::{Config, Readability};

#[test]
fn test_candidates() {
    // This test demonstrates that with a deeply nested structure,
    // the original Readability candidate selection mode may fail
    // to extract all meaningful content.
    //
    // In this document each paragraph is wrapped in several nested <div>s.
    // Because of that, the original Readability algorithm may fail to
    // accumulate enough score for the common ancestor and only one
    // paragraph is selected.
    //
    // The `DomSmoothie` approach performs better here because it
    // does not rely on the additional ancestor candidate lists
    // required by the original Readability algorithm.
    
    let contents = include_str!("../test-pages/alice-two-paragraphs.html");
    
    let first_p_mark = "Alice was beginning to get very tired of sitting";
    let second_p_mark = "So she was considering in her own mind";

    let mut ra = Readability::new(contents, None, None).unwrap();
    let res = ra.parse_with_policy(dom_smoothie::ParsePolicy::Strict).unwrap();
    
    
    assert!(res.text_content.contains(first_p_mark));
    assert!(!res.text_content.contains(second_p_mark));
    
    let cfg = Config{
        candidate_select_mode: dom_smoothie::CandidateSelectMode::DomSmoothie,
        ..Default::default()
    };
    
    let mut ra = Readability::new(contents, None, Some(cfg)).unwrap();
    let res = ra.parse_with_policy(dom_smoothie::ParsePolicy::Strict).unwrap();
    
    assert!(res.text_content.contains(first_p_mark));
    assert!(res.text_content.contains(second_p_mark));
    
}