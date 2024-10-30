use dom_query::Node;

use crate::glob::*;

const SCORE_ATTR: &str = "data-readability-score";

pub(crate) fn get_node_score(node: &Node) -> Option<f32> {
    let score = node.attr(SCORE_ATTR);
    if let Some(score) = score {
        if let Ok(score) = score.parse::<f32>() {
            return Some(score);
        }
    }
    None
}

pub(crate) fn has_node_score(node: &Node) -> bool {
    node.has_attr(SCORE_ATTR)
}

pub(crate) fn set_node_score(node: &Node, score: f32) {
    node.set_attr(SCORE_ATTR, &score.to_string());
}

pub(crate) fn init_node_score(node: &Node) -> f32 {
    let score = determine_node_score(node);
    set_node_score(node, score);
    score
}

pub(crate) fn determine_node_score(node: &Node) -> f32 {
    let node_name = node.node_name().unwrap();

    let score: f32 = match node_name.as_ref() {
        "div" => 5.0,
        "pre" | "td" | "blockquote" => 3.0,
        "address" | "ol" | "ul" | "dl" | "dd" | "dt" | "li" | "form" => -3.0,
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "th" => -5.0,
        _ => 0.0,
    };

    score + get_class_weight(node)
}

pub(crate) fn get_class_weight(node: &Node) -> f32 {
    // flag.FLAG_WEIGHT_CLASSES, ignore now

    let mut weight = 0.0;

    let class_name = node.attr("class");

    if let Some(class_name) = class_name {
        if RX_CLASSES_NEGATIVE.is_match(&class_name) {
            weight -= 25.0;
        }
        if RX_CLASSES_POSITIVE.is_match(&class_name) {
            weight += 25.0;
        }
    }

    let id = node.attr("id");
    if let Some(id) = id {
        if RX_CLASSES_NEGATIVE.is_match(&id) {
            weight -= 25.0;
        }
        if RX_CLASSES_POSITIVE.is_match(&id) {
            weight += 25.0;
        }
    }

    weight
}