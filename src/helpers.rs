use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

use dom_query::{Node, Selection};

use crate::glob::*;
use crate::matching::*;

pub(crate) fn text_similarity(text_a: &str, text_b: &str) -> f64 {
    //TODO: revise this later (use Jaccard index)
    if text_a.is_empty() || text_b.is_empty() {
        return 0.0;
    }

    let a = text_a.to_lowercase();
    let b = text_b.to_lowercase();

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
    let Some(qual_name) = node.qual_name_ref() else {
        return false;
    };
    let node_name = qual_name.local.as_ref();
    if PHRASING_ELEMS.contains(node_name) {
        return true;
    }

    if matches!(node_name, "a" | "del" | "ins") {
        // There is no big sense to consider link content as phrasing content if they doesn't have children element.
        let children = node.children();
        return !children.is_empty() && children.into_iter().all(|n| is_phrasing_content(&n));
    }
    false
}

pub(crate) fn is_whitespace(node: &Node) -> bool {
    if node.is_text() && !node.is_nonempty_text() {
        return true;
    }
    // only an element node has a node_name
    MINI_BR.match_node(node)
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
    match filter_fn {
        Some(f) => node.ancestors_it(max_depth).any(|a| a.has_name(tag) && f(&a)),
        None => node.ancestors_it(max_depth).any(|a| a.has_name(tag)),
    }
}

pub(crate) fn get_text_density(node: &Node, selector: &str, char_count: Option<usize>) -> f32 {
    let sel = Selection::from(node.clone()).select(selector);
    let sel_nodes = sel.nodes();

    if sel_nodes.is_empty() {
        return 0.0;
    }

    let text_length = if let Some(c) = char_count {
        c as f32
    } else {
        node.normalized_char_count() as f32
    };
    if text_length == 0.0 {
        return 0.0;
    }

    let children_length = sel_nodes
        .iter()
        .map(|n| n.normalized_char_count() as f32)
        .sum::<f32>();
    children_length / text_length
}

pub(crate) fn normalize_spaces(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut iter = text.split_whitespace();

    if let Some(first) = iter.next() {
        result.push_str(first);
        for word in iter {
            result.push(' ');
            result.push_str(word);
        }
    }
    result
}

pub(crate) fn link_density(node: &Node, char_count: Option<usize>) -> f32 {
    let mut link_length = 0f32;

    for a in node.find_descendants("a") {
        let href = a.attr_or("href", "");
        let coeff = if href.len() > 1 && href.starts_with('#') {
            0.3
        } else {
            1.0
        };
        link_length += a.normalized_char_count() as f32 * coeff;
    }

    if link_length == 0.0 {
        return 0.0;
    }

    let text_length = if let Some(c) = char_count {
        c as f32
    } else {
        node.normalized_char_count() as f32
    };
    if text_length == 0.0 {
        return 0.0;
    }

    link_length / text_length
}

pub(crate) fn has_single_tag_inside_element(node: &Node, tag: &str) -> bool {
    // There should be exactly 1 element child with given tag
    let children = node.element_children();
    if children.len() != 1 {
        return false;
    }

    if !children.first().map_or(false, |child| child.has_name(tag)) {
        return false;
    }

    !node.children_it(false).any(|n| n.is_nonempty_text())
}

pub(crate) fn is_element_without_content(node: &Node) -> bool {
    // since this function calls only for elements check `node.is_element()` is redundant
    let has_text = node.descendants_it().any(|n| n.is_nonempty_text());
    if has_text {
        return false;
    }

    let el_children_count = node.children_it(false).filter(|n| n.is_element()).count();
    if el_children_count == 0 {
        return true;
    }

    let line_breaks = node.find_descendants("br").len() + node.find_descendants("hr").len();
    el_children_count == line_breaks
}

pub(crate) fn get_dir_attr(node: &Node) -> Option<String> {
    if let Some(first_child) = node.first_child() {
        if let Some(dir_attr) = first_child.attr("dir") {
            return Some(dir_attr.to_string());
        }

        let dir_attr = first_child.ancestors_it(None).find_map(|a| a.attr("dir"));
        if let Some(dir_attr) = dir_attr {
            return Some(dir_attr.to_string());
        }
    }
    None
}

pub(crate) fn node_name_in(node: &Node, names: &phf::Set<&str>) -> bool {
    node.qual_name_ref()
        .map_or(false, |name| names.contains(name.local.as_ref()))
}

pub(crate) fn is_probably_visible(node: &Node) -> bool {
    if node.has_attr("hidden") {
        return false;
    }
    if is_invisible_style(node) {
        return false;
    }
    if MINI_FALLBACK_IMG.match_node(node) {
        return true;
    }
    if MINI_ARIA_HIDDEN.match_node(node) {
        return false;
    }
    true
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
