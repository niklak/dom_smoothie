use dom_smoothie::{ParsePolicy, Readability};

use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};

fn hash_text<T: Hash>(text: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

#[test]
pub(crate) fn test_parse_with_policy() -> Result<(), Box<dyn Error>> {
    // this is a case when each policy produces a different result
    let source_contents = include_str!("../test-pages/ok/wikipedia-2/source.html");

    let policies: [ParsePolicy; 4] = [
        ParsePolicy::Strict,
        ParsePolicy::Moderate,
        ParsePolicy::Clean,
        ParsePolicy::Raw,
    ];
    let mut results = vec![];

    for policy in policies {
        let mut r = Readability::new(source_contents, None, None)?;
        let article = r.parse_with_policy(policy)?;
        let content_hash = hash_text(&article.content.trim());
        if !results.contains(&content_hash) {
            results.push(content_hash);
        }
    }
    assert_eq!(results.len(), policies.len());
    Ok(())
}

#[test]
pub(crate) fn test_parse_with_policy_fail() -> Result<(), Box<dyn Error>> {
    // Test that problematic HTML fails with Strict policy
    let source_contents = include_str!("../test-pages/readability/lazy-image-3/source.html");
    let mut r = Readability::new(source_contents, None, None)?;
    let article = r.parse_with_policy(ParsePolicy::Strict);
    assert!(article.is_err());

    let mut r = Readability::new(source_contents, None, None)?;
    let article = r.parse_with_policy(ParsePolicy::Raw);
    assert!(article.is_ok());
    Ok(())
}
