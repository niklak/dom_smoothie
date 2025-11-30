use dom_query::{NodeData, NodeRef};

#[allow(clippy::wildcard_imports)]
use crate::glob::*;
use crate::matching::contains_one_of_words;

pub(crate) fn get_node_score(node: &NodeRef) -> f32 {
    node.attr(SCORE_ATTR)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0)
}

pub(crate) fn has_node_score(node: &NodeRef) -> bool {
    node.has_attr(SCORE_ATTR)
}

pub(crate) fn set_node_score(node: &NodeRef, score: f32) {
    node.set_attr(SCORE_ATTR, &score.to_string());
}

pub(crate) fn init_node_score(node: &NodeRef, weigh_classes: bool) -> f32 {
    let score = determine_node_score(node, weigh_classes);
    set_node_score(node, score);
    score
}

pub(crate) fn determine_node_score(node: &NodeRef, weigh_classes: bool) -> f32 {
    let Some(node_name) = node.qual_name_ref() else {
        return 0.0;
    };

    let score: f32 = match node_name.local.as_ref() {
        "div" => 5.0,
        "pre" | "td" | "blockquote" => 3.0,
        "address" | "ol" | "ul" | "dl" | "dd" | "dt" | "li" | "form" => -3.0,
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "th" => -5.0,
        _ => 0.0,
    };

    score + get_class_weight(node, weigh_classes)
}

pub(crate) fn get_class_weight(node: &NodeRef, weigh_classes: bool) -> f32 {
    let mut weight = 0.0;

    if !weigh_classes {
        return weight;
    }

    if let Some(el) = node.element_ref() {
        if let Some(mut class_name) = el.class() {
            class_name.make_ascii_lowercase();
            weight += determine_attr_weight(&class_name);
        }

        if let Some(mut id_attr) = el.id() {
            id_attr.make_ascii_lowercase();
            weight += determine_attr_weight(&id_attr);
        }
    }

    weight
}

#[cfg(not(feature = "aho-corasick"))]
fn determine_attr_weight(attr: &str) -> f32 {
    use crate::helpers::BytePatternCheck;
    let mut weight: f32 = 0.0;

    let check = BytePatternCheck::new(attr);
    if check.contains_any(CLASSES_NEGATIVE) || contains_one_of_words(attr, CLASSES_NEGATIVE_WORDS) {
        weight -= 25.0;
    }
    if check.contains_any(CLASSES_POSITIVE) {
        weight += 25.0;
    }
    weight
}

#[cfg(feature = "aho-corasick")]
fn determine_attr_weight(attr: &str) -> f32 {
    let mut weight: f32 = 0.0;
    if crate::ac_automat::AC_CLASSES_NEGATIVE.is_match(attr)
        || contains_one_of_words(attr, CLASSES_NEGATIVE_WORDS)
    {
        weight -= 25.0;
    }
    if crate::ac_automat::AC_CLASSES_POSITIVE.is_match(attr) {
        weight += 25.0;
    }
    weight
}

pub(crate) fn score_text_content(node: &NodeRef) -> usize {
    node.descendants_it()
        .filter_map(|n| {
            n.query(|tree_node| {
                if let NodeData::Text { ref contents } = tree_node.data {
                    contents.chars().filter(|c| COMMAS.contains(c)).count()
                } else {
                    0
                }
            })
        })
        .sum()
}
