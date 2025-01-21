use dom_query::Document;

use tendril::format_tendril;

use crate::glob::*;
use crate::helpers::is_probably_visible;

/// Estimates whether the document is readable in a *quick-and-dirty* way.
///
///
/// # Arguments
///
/// * `doc` - The reference to the [`dom_query::Document`] to check.
/// * `min_score` - The minimum score required for the document to be considered readable. Defaults to 20.0.
/// * `min_content_length` - The minimum content length required for the document to be considered readable. Defaults to 140.
///
/// # Returns
///
/// True if the document is readable, false otherwise.
pub fn is_probably_readable(
    doc: &Document,
    min_score: Option<f32>,
    min_content_length: Option<usize>,
) -> bool {
    let min_score = min_score.unwrap_or(MIN_SCORE);
    let min_content_length = min_content_length.unwrap_or(MIN_CONTENT_LENGTH);

    let mut nodes = doc.select("p,pre,article").nodes().to_vec();

    let br_parent_sel = doc.select("div > br").parent();
    let br_parent_nodes = br_parent_sel.nodes();
    nodes.extend_from_slice(br_parent_nodes);

    let mut score: f32 = 0.0;

    nodes.iter().any(|node| {
        if !is_probably_visible(node) {
            return false;
        }
        let match_string =
            format_tendril!("{} {}", node.attr_or("class", ""), node.attr_or("id", ""));

        if UNLIKELY_CANDIDATES.iter().any(|p| match_string.contains(p))
            && !MAYBE_CANDIDATES.iter().any(|p| match_string.contains(p))
        {
            return false;
        }

        if MATCHER_LI_P.match_element(node) {
            return false;
        }

        let text_content_length = node.text().trim().chars().count();
        if text_content_length < min_content_length {
            return false;
        }

        score += ((text_content_length - min_content_length) as f32).sqrt();
        if score > min_score {
            return true;
        }
        false
    })
}
