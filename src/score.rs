use dom_query::{NodeData, NodeRef};

#[allow(clippy::wildcard_imports)]
use crate::glob::*;
use crate::matching::contains_one_of_words;

pub(crate) fn get_node_score(node: &NodeRef) -> f32 {
    node.attr(SCORE_ATTR)
        .and_then(|s| s.parse::<u32>().ok())
        .map_or(0.0, f32::from_bits)
}

pub(crate) fn has_node_score(node: &NodeRef) -> bool {
    node.has_attr(SCORE_ATTR)
}

pub(crate) fn set_node_score(node: &NodeRef, score: f32) {
    let mut buf = itoa::Buffer::new();
    node.set_attr(SCORE_ATTR, buf.format(score.to_bits()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use dom_query::Document;

    #[test]
    fn test_set_get_node_score_round_trip() {
        let doc = Document::from("<div id=\"a\"></div><div id=\"b\"></div>");
        let a = doc.select("#a").nodes()[0];
        let b = doc.select("#b").nodes()[0];

        assert_eq!(get_node_score(&b).to_bits(), 0u32); // untouched node

        for score in [
            0.0,
            1.0,
            -3.0,
            27.5,
            123.456_78,
            f32::MAX,
            f32::MIN,
            f32::MIN_POSITIVE,
            f32::EPSILON,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            f32::from_bits(1), // subnormal
            f32::from_bits(0x7fc0_1234), // non-canonical NaN payload
        ] {
            set_node_score(&a, score);
            // Bit-exact: NaN payloads and signs survive the attribute round-trip.
            assert_eq!(get_node_score(&a).to_bits(), score.to_bits());
        }
    }
}
