use dom_query::{Node, NodeData};

use crate::{glob::*, matching::contains_one_of_words};

pub(crate) fn get_node_score(node: &Node) -> f32 {
    let score = node.attr(SCORE_ATTR);
    if let Some(Ok(score)) = score.map(|s| s.parse::<f32>()) {
        return score;
    }
    0.0
}

pub(crate) fn has_node_score(node: &Node) -> bool {
    node.has_attr(SCORE_ATTR)
}

pub(crate) fn set_node_score(node: &Node, score: f32) {
    node.set_attr(SCORE_ATTR, &score.to_string());
}

pub(crate) fn init_node_score(node: &Node, weigh_classes: bool) -> f32 {
    let score = determine_node_score(node, weigh_classes);
    set_node_score(node, score);
    score
}

pub(crate) fn determine_node_score(node: &Node, weigh_classes: bool) -> f32 {
    let Some(node_name) = node.tree.get_name(&node.id) else {
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

pub(crate) fn get_class_weight(node: &Node, weigh_classes: bool) -> f32 {
    let mut weight = 0.0;

    if !weigh_classes {
        return weight;
    }

    node.query(|n| {
        if let NodeData::Element(ref el) = n.data {
            if let Some(mut class_name) = el.class() {
                class_name.make_ascii_lowercase();
                weight += determine_attr_weight(&class_name);
            };

            if let Some(mut id_attr) = el.id() {
                id_attr.make_ascii_lowercase();
                weight += determine_attr_weight(&id_attr);
            }
        }
    });

    weight
}

fn determine_attr_weight(attr: &str) -> f32 {
    let mut weight: f32 = 0.0;
    if CLASSES_NEGATIVE.iter().any(|pat| attr.contains(pat))
        || contains_one_of_words(attr, &CLASSES_NEGATIVE_WORDS)
    {
        weight -= 25.0;
    }
    if CLASSES_POSITIVE.iter().any(|pat| attr.contains(pat)) {
        weight += 25.0;
    }
    weight
}
