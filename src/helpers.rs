use std::collections::HashSet;

use foldhash::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use dom_query::{NodeId, NodeRef, Selection};

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

    // TODO: can be calculated without joining
    let merged_b = tokens_b.join(" ");
    let merged_unique_b = unique_tokens_b.join(" ");

    let distance_b = merged_unique_b.chars().count() as f64 / merged_b.chars().count() as f64;
    1.0 - distance_b
}

pub(crate) fn is_phrasing_content(node: &NodeRef) -> bool {
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

pub(crate) fn is_whitespace(node: &NodeRef) -> bool {
    if node.is_text() && !node.is_nonempty_text() {
        return true;
    }
    // only an element node has a node_name
    node.has_name("br")
}

pub(crate) fn has_ancestor<F>(node: &NodeRef, max_depth: Option<usize>, filter_fn: F) -> bool
where
    F: Fn(&NodeRef) -> bool,
{
    let max_depth = max_depth.map(|max_depth| if max_depth == 0 { 3 } else { max_depth });
    node.ancestors_it(max_depth).any(|a| filter_fn(&a))
}

pub(crate) fn text_density(node: &NodeRef, selector: &str, char_count: Option<usize>) -> f32 {
    let sel = Selection::from(*node).select(selector);
    let sel_nodes = sel.nodes();

    let children_length: f32 = sel_nodes
        .iter()
        .map(|n| n.normalized_char_count())
        .sum::<usize>() as f32;

    if children_length == 0.0 {
        return 0.0;
    }
    let text_length = char_count.unwrap_or_else(|| node.normalized_char_count()) as f32;

    if text_length == 0.0 {
        return 0.0;
    }
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

pub(crate) fn link_density_fn<F>(node: &NodeRef, char_count: Option<usize>, mut count_fn: F) -> f32
where
    F: FnMut(&NodeRef) -> usize,
{
    let mut link_length = 0f32;

    for a in node.find_descendants("a") {
        let href = a.attr_or("href", "");
        let coeff = if href.len() > 1 && href.starts_with('#') {
            0.3
        } else {
            1.0
        };
        link_length += count_fn(&a) as f32 * coeff;
    }

    if link_length == 0.0 {
        return 0.0;
    }

    let text_length = char_count.unwrap_or_else(|| count_fn(node)) as f32;
    if text_length == 0.0 {
        return 0.0;
    }

    link_length / text_length
}

pub(crate) fn link_density(node: &NodeRef, char_count: Option<usize>) -> f32 {
    link_density_fn(node, char_count, |n| n.normalized_char_count())
}

pub(crate) fn has_single_tag_inside_element(node: &NodeRef, tag: &str) -> bool {
    // There should be exactly 1 element child with given tag
    let children = node.element_children();
    if children.len() != 1 {
        return false;
    }

    if !children.first().is_some_and(|child| child.has_name(tag)) {
        return false;
    }

    !node.children_it(false).any(|n| n.is_nonempty_text())
}

pub(crate) fn is_element_without_content(node: &NodeRef) -> bool {
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

pub(crate) fn get_dir_attr(node: &NodeRef) -> Option<String> {
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

pub(crate) fn node_name_in(node: &NodeRef, names: &phf::Set<&str>) -> bool {
    node.qual_name_ref()
        .is_some_and(|name| names.contains(name.local.as_ref()))
}

pub(crate) fn is_probably_visible(node: &NodeRef) -> bool {
    if node.has_attr("hidden") {
        return false;
    }
    if is_invisible_style(node) {
        return false;
    }
    !MINI_ARIA_HIDDEN.match_node(node) || MINI_FALLBACK_IMG.match_node(node)
}

#[cfg(not(feature = "aho-corasick"))]
/// A lightweight ASCII-only pre-checker used to quickly skip patterns
/// that cannot occur in the haystack.
pub(crate) struct AsciiPatternCheck<'a> {
    haystack: &'a str,
    char_map: [u8; 256],
}

#[cfg(not(feature = "aho-corasick"))]
impl<'a> AsciiPatternCheck<'a> {
    pub(crate) fn new(haystack: &'a str) -> AsciiPatternCheck<'a> {
        let mut char_map = [0u8; 256];

        for &b in haystack.as_bytes() {
            char_map[b as usize] = 1;
        }
        AsciiPatternCheck { haystack, char_map }
    }
    #[inline]
    fn pre_check(&self, pat: &str) -> bool {
        for &b in pat.as_bytes() {
            if self.char_map[b as usize] == 0 {
                return false;
            }
        }
        true
    }
    /// Checks if the haystack contains any of the given patterns.
    /// Performs a cheap ASCII bitmap pre-check before `str::contains`.
    pub(crate) fn contains_any(&self, pats: &[&str]) -> bool {
        pats.iter()
            .any(|pat| self.pre_check(pat) && self.haystack.contains(pat))
    }
}

#[derive(Default)]
pub(crate) struct CharCounterCache {
    inner: HashMap<NodeId, usize>,
}

impl CharCounterCache {
    pub(crate) fn char_count(&mut self, node: &NodeRef) -> usize {
        *self
            .inner
            .entry(node.id)
            .or_insert_with(|| node.normalized_char_count())
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

    #[test]
    fn test_normalize_spaces() {
        let text = "    The quick  brown  fox\n jumps over the lazy dog. ";
        let normalized_text = normalize_spaces(text);
        let expected = "The quick brown fox jumps over the lazy dog.";
        assert_eq!(expected, normalized_text);
    }
}
