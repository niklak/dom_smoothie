use std::collections::HashSet;

use dom_query::{Node, Selection};

use crate::glob::*;

pub(crate) fn text_similarity(text_a: &str, text_b: &str) -> f64 {
    //TODO: revise this later (use Jaccard index)
    let a = text_a.to_lowercase();
    let b = text_b.to_lowercase();
    let unique_tokens_a: HashSet<&str> = RX_TOKENIZE.split(&a).filter(|s| !s.is_empty()).collect();

    let tokens_b: Vec<&str> = RX_TOKENIZE.split(&b).filter(|s| !s.is_empty()).collect();

    let unique_tokens_b: Vec<&str> = tokens_b
        .iter()
        .filter(|&&s| !unique_tokens_a.contains(s))
        .cloned()
        .collect();

    let merged_b = tokens_b.join(" ");
    let merged_unique_b = unique_tokens_b.join(" ");

    let distance_b = merged_unique_b.chars().count() as f64 / merged_b.chars().count() as f64;
    1.0 - distance_b
}

pub(crate) fn is_phrasing_content(node: &Node) -> bool {
    // TODO: revise this function
    if node.is_text() {
        return true;
    }

    if !node.is_element() {
        return false;
    }

    let node_name_t = node.node_name().unwrap();
    let node_name: &str = &node_name_t;
    if PHRASING_ELEMS.contains(&node_name) {
        return true;
    }

    if (node_name == "a")
        || (node_name == "del")
        || (node_name == "ins")
        || node.children().into_iter().all(|n| is_phrasing_content(&n))
    {
        return true;
    }

    false
}

pub(crate) fn is_whitespace(node: &Node) -> bool {
    if node.is_text() {
        return node.text().trim().is_empty();
    }
    if node.is_element() {
        return node.node_name().map_or(false, |name| name == "br".into());
    }
    false
}

pub(crate) type NodePredicate = fn(&Node) -> bool;

pub(crate) fn has_ancestor_tag<F>(
    node: &Node,
    tag: &str,
    max_depth: Option<usize>,
    filter_fn: Option<F>,
) -> bool
where
    F: Fn(&Node) -> bool,
{
    //TODO: revise this with node.ancestors()!
    let max_depth = max_depth.map(|max_depth| if max_depth == 0 { 3 } else { max_depth });
    let mut matched_ancestors = node
        .ancestors_it(max_depth)
        .filter(|a| a.is_element())
        .filter(|a| a.node_name().unwrap().as_ref() == tag);

    if let Some(filter_fn) = filter_fn {
        matched_ancestors.any(|a| filter_fn(&a))
    } else {
        matched_ancestors.count() > 0
    }
}

pub fn get_text_density(node: &Node, selector: &str) -> f32 {
    let text_length = node.text().chars().count() as f32;
    if text_length == 0.0 {
        return 0.0;
    }
    let sel = Selection::from(node.clone()).select(selector);
    let children_length =  normalize_spaces(&sel.text()).chars().count() as f32;
    children_length / text_length
}

pub fn normalize_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn link_density(node: &Node) -> f32 {
    let text_length: f32 = node.text().chars().count() as f32;
    if text_length == 0.0 {
        return 0.0;
    }
    let mut link_length = 0f32;

    let a_sel = Selection::from(node.clone()).select("a");

    for a in a_sel.iter() {
        let href = a.attr_or("href", "");
        let coeff = if !href.is_empty() && RX_HASH_URL.is_match(href.as_ref()) {
            0.3
        } else {
            1.0
        };
        link_length += a.text().len() as f32 * coeff;
    }

    link_length / text_length
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_text_similarity() {
        let text_a = "The quick brown fox";
        let text_b = "The quick fox";
        let similarity = text_similarity(text_a, text_b);
        assert!(similarity > 0.75);
    }

    #[test]
    fn test_text_similarity_similar() {
        let text_a = "THE QUICK FOX";
        let text_b = "The quick fox";
        let similarity = text_similarity(text_a, text_b);
        assert!(similarity == 1.0);
    }
}
