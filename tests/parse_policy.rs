use dom_smoothie::{ParsePolicy, Readability};

use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

fn hash_text<T: Hash>(text: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

#[test]
pub(crate) fn test_parse_with_policy() {
    // this is a case when each policy produce a different result
    let source_path = Path::new("./test-pages/ok/wikipedia-2/source.html");
    let source_contents = fs::read_to_string(source_path).unwrap();

    let policies: [ParsePolicy; 4] = [
        ParsePolicy::Strict,
        ParsePolicy::Moderate,
        ParsePolicy::Clean,
        ParsePolicy::Raw,
    ];
    let mut results = vec![];

    for policy in policies {
        let mut r = Readability::new(source_contents.clone(), None, None).unwrap();
        let article = r.parse_with_policy(policy).unwrap();
        let content_hash = hash_text(&article.content.trim());
        if !results.contains(&content_hash) {
            results.push(content_hash);
        }
    }
    assert_eq!(results.len(), policies.len());
}
