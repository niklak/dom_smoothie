use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

use dom_query::{Node, Selection};

use crate::glob::*;

pub(crate) fn text_similarity(text_a: &str, text_b: &str) -> f64 {
    //TODO: revise this later (use Jaccard index)
    let a = text_a.to_lowercase();
    let b = text_b.to_lowercase();

    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    if a.contains(&b) {
        return 1.0;
    }

    let unique_tokens_a: HashSet<&str> = a.unicode_words().collect();

    let tokens_b: Vec<&str> = b.unicode_words().collect();
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
    if node.is_text() {
        return true;
    }

    // only elements has a node name
    let Some(node_name) = node.node_name() else {
        return false;
    };

    if PHRASING_ELEMS.contains(&node_name.as_ref()) {
        return true;
    }

    if matches!(node_name.as_ref(), "a" | "del" | "ins")
        && node.children().into_iter().all(|n| is_phrasing_content(&n))
    {
        return true;
    }

    false
}

pub(crate) fn is_whitespace(node: &Node) -> bool {
    if node.is_text() {
        return node.text().trim().is_empty();
    }
    // only an element node has a node_name
    node.node_name().map_or(false, |name| name == "br".into())
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
    let max_depth = max_depth.map(|max_depth| if max_depth == 0 { 3 } else { max_depth });
    let mut matched_ancestors = node
        .ancestors_it(max_depth)
        .filter(|a| match a.node_name() {
            Some(name) => name.as_ref() == tag,
            None => false,
        });

    if let Some(filter_fn) = filter_fn {
        matched_ancestors.any(|a| filter_fn(&a))
    } else {
        matched_ancestors.count() > 0
    }
}

pub fn get_text_density(node: &Node, selector: &str) -> f32 {
    let text_length = normalize_spaces(&node.text()).chars().count() as f32;
    if text_length == 0.0 {
        return 0.0;
    }
    let sel = Selection::from(node.clone()).select(selector);
    let children_length = normalize_spaces(&sel.text()).chars().count() as f32;
    children_length / text_length
}

pub fn normalize_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn link_density(node: &Node) -> f32 {
    let text_length: f32 = normalize_spaces(&node.text()).chars().count() as f32;
    if text_length == 0.0 {
        return 0.0;
    }
    let mut link_length = 0f32;

    let a_sel = Selection::from(node.clone()).select_matcher(&MATCHER_A);

    for a in a_sel.iter() {
        let href = a.attr_or("href", "");
        let coeff = if !href.is_empty() && RX_HASH_URL.is_match(href.as_ref()) {
            0.3
        } else {
            1.0
        };
        link_length += normalize_spaces(&a.text()).chars().count() as f32 * coeff;
    }

    link_length / text_length
}

pub(crate) fn has_single_tag_inside_element(node: &Node, tag: &str) -> bool {
    // There should be exactly 1 element child with given tag
    let children = node.element_children();
    if children.len() != 1 {
        return false;
    }

    let first_child = children.first();

    if !first_child
        .and_then(|child| child.node_name())
        .map_or(false, |name| name.as_ref() == tag)
    {
        return false;
    }

    !node
        .children()
        .iter()
        .any(|n| n.is_text() && RX_HAS_CONTENT.is_match(n.text().as_ref()))
}

pub(crate) fn is_element_without_content(node: &Node) -> bool {
    let is_element = node.is_element();
    let no_text = node.text().trim().is_empty();
    let no_element_children = node.element_children().is_empty();

    let sel = Selection::from(node.clone()).select_matcher(&MATCHER_BR_HR);
    is_element && no_text && (no_element_children || node.element_children().len() == sel.length())
}

pub(crate) fn set_dir_attr(node: &Node) {
    if let Some(first_child) = node.first_child() {
        if first_child.has_attr("dir") {
            return;
        }

        let dir_attr = first_child.ancestors_it(None).find_map(|a| a.attr("dir"));
        if let Some(dir_attr) = dir_attr {
            node.set_attr("dir", dir_attr.as_ref());
        }
    }
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
    fn test_text_similarity_contains() {
        let text_a = "the quick brown fox jumps over the lazy dog";
        let text_b = "The Quick Brown Fox";
        let similarity = text_similarity(text_a, text_b);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_text_similarity_similar() {
        let text_a = "DeepMind新电脑已可利用记忆自学 人工智能迈上新台阶_科技_腾讯网";
        let text_b = "DeepMind新电脑已可利用记忆自学 人工智能迈上新台阶";
        let similarity = text_similarity(text_a, text_b);
        assert_eq!(similarity, 1.0);
    }
}
