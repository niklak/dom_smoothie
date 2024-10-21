use std::vec;

use dom_query::{Document, Node, NodeData, NodeRef, Selection};
use tendril::StrTendril;

use crate::glob::*;
use crate::score::*;

use crate::helpers::{is_phrasing_content, is_whitespace, text_similarity};
use crate::MetaData;
//TODO: do not forget FLAGS

pub fn grab_article(doc: &Document, metadata: Option<MetaData>) {
    let mut metadata = metadata.unwrap_or_default();

    clean_doc(doc);

    let mut elements_to_score: Vec<Node> = vec![];

    if !metadata.title.is_empty() {
        // if title is not empty then delete duplicate
        remove_header_duplicates_title(doc, &metadata.title);
    }

    let selection = doc.select("*");

    //TODO: maybe this way of iterating through nodes is not the best
    for node in selection.nodes() {
        if !node.is_element() {
            continue;
        }

        if !is_probably_visible(node) {
            node.remove_from_parent();
            continue;
        }
        //TODO: byline may be optimized
        let match_string = get_node_matching_string(node);
        if metadata.byline.is_empty() && is_valid_byline(node, &match_string) {
            metadata.byline = node.text().trim().to_string();
            node.remove_from_parent();
            continue;
        }

        if is_unlikely_candidate(node, &match_string) {
            node.remove_from_parent();
            continue;
        }

        let node_name = node.node_name().unwrap();

        if DEFAULT_TAGS_TO_SCORE.contains(&node_name.as_ref()) {
            elements_to_score.push(node.clone());
        }

        // TODO: div_matcher.match_element(node)

        if node_name.as_ref() == "div" {
            div_into_p(node, doc, &mut elements_to_score);
        }
    }

    handle_candidates(&mut elements_to_score, doc);

    //TODO: handle elements_to_score
}

fn clean_doc(doc: &Document) {
    //remove by selection in any case
    // User is not able to see elements applied with both "aria-modal = true" and "role = dialog"
    doc.select_matcher(&DIALOGS_MATCHER).remove();

    // Remove DIV, SECTION, and HEADER nodes without any content(e.g. text, image, video, or iframe).
    doc.select_matcher(&EMPTY_SECTION_MATCHER).remove();

    for node in doc.select_matcher(&ROLES_MATCHER).nodes() {
        if let Some(role) = node.attr("role") {
            if UNLIKELY_ROLES.contains(&role.as_ref()) {
                node.remove_from_parent();
            }
        }
    }
}

fn get_node_matching_string(node: &NodeRef<NodeData>) -> String {
    let mut matched_attrs: Vec<String> = vec![];
    let class = node.attr("class");
    let id = node.attr("id");
    if let Some(class) = class {
        matched_attrs.push(class.to_string());
    }

    if let Some(id) = id {
        matched_attrs.push(id.to_string());
    }

    matched_attrs.join(" ")
}

fn is_probably_visible(node: &Node) -> bool {
    if node.has_attr("hidden") {
        return false;
    }

    let is_invisible_style = node
        .attr("style")
        .map_or(false, |s| RX_STYLE_DISPLAY_NONE.is_match(&s));

    if is_invisible_style {
        return false;
    }

    let is_aria_hidden = node
        .attr("aria-hidden")
        .map_or(false, |a| a.as_ref() == "true");
    let has_fallback_image = node
        .attr("class")
        .map_or(false, |s| s.contains("fallback-image"));

    if is_aria_hidden && !has_fallback_image {
        return false;
    }

    true
}

fn is_valid_byline(node: &Node, match_string: &str) -> bool {
    let byline_len = node.text().trim().chars().count();
    if byline_len > 100 && byline_len != 0 {
        return false;
    }

    if let Some(rel) = node.attr("rel") {
        if rel.as_ref() == "author" {
            return true;
        }
    }

    if let Some(itemprop) = node.attr("itemprop") {
        if itemprop.contains("author") {
            return true;
        }
    }

    RX_BYLINE.is_match(match_string)
}

// Removes the first occurred title duplicate from the document
fn remove_header_duplicates_title(doc: &Document, title: &str) {
    for sel in doc.select_matcher(&HEADINGS_MATCHER).iter() {
        let heading = sel.text();
        if text_similarity(title, heading.trim()) > 0.75 {
            sel.remove();
            return;
        }
    }
}

fn is_unlikely_candidate(node: &Node, match_string: &str) -> bool {
    if !RX_UNLIKELY_CANDIDATES.is_match(match_string) {
        return false;
    }
    if RX_MAYBE_CANDIDATES.is_match(match_string) {
        return false;
    }

    let name = node.node_name().unwrap();
    if name.as_ref() == "a" || name.as_ref() == "body" {
        return false;
    }
    if has_ancestor_tag::<NodePredicate>(node, "table", None, None) {
        return false;
    }
    if has_ancestor_tag::<NodePredicate>(node, "code", None, None) {
        return false;
    }
    true
}

type NodePredicate = fn(&Node) -> bool;

fn has_ancestor_tag<F>(
    node: &Node,
    tag: &str,
    max_depth: Option<usize>,
    filter_fn: Option<F>,
) -> bool
where
    F: Fn(&Node) -> bool,
{

    //TODO: revise this with node.ancestors()!
    let max_depth = max_depth.unwrap_or(3);
    if max_depth == 0 {
        return false;
    }
    let mut depth: usize = 0;

    let mut parent_node = node.parent();
    while let Some(ref parent) = parent_node {
        if depth > max_depth {
            break;
        }
        // if node has no name, then it is not element, skip it
        if let Some(name) = parent.node_name() {
            if name.as_ref() == tag && filter_fn.as_ref().map_or(true, |f| f(parent)) {
                return true;
            }
        } else {
            parent_node = parent.parent();
            continue;
        }
        parent_node = parent.parent();
        depth += 1;
    }

    false
}

fn div_into_p<'a>(
    node: &'a Node,
    doc: &'a Document,
    elements_to_score: &mut Vec<NodeRef<'a, NodeData>>,
) {
    // Turn all divs that don't have children block level elements into p's

    // Put phrasing content into paragraphs.
    let mut p_node: Option<Node> = None;
    let mut child_node = node.first_child();

    while let Some(ref child) = child_node {
        if is_phrasing_content(child) {
            if let Some(ref p) = p_node {
                p.append_child(&child.id);
            } else if !is_whitespace(child) {
                let raw_p = doc.tree.new_element("p");
                child.append_prev_sibling(&raw_p.id);
                child.remove_from_parent();
                raw_p.append_child(&child.id);
                p_node = Some(raw_p);
            }
        } else if let Some(ref p) = p_node {
            //TODO: careful! Revise this:
            while let Some(p_last_child) = p.last_child() {
                if is_whitespace(&p_last_child) {
                    p_last_child.remove_from_parent();
                } else {
                    break;
                }
            }
            p_node = None;
        }
        child_node = child.next_sibling();
    }

    // Sites like http://mobile.slate.com encloses each paragraph with a DIV
    // element. DIVs with only a P element inside and no text content can be
    // safely converted into plain P elements to avoid confusing the scoring
    // algorithm with DIVs with are, in practice, paragraphs.

    if has_single_tag_inside_element(node, "p") && link_density(node) < 0.25 {
        let new_node = node.first_child().unwrap();
        node.append_prev_sibling(&new_node.id);
        node.remove_from_parent();
        elements_to_score.push(new_node.clone());
    } else if !has_child_block_element(node) {
        node.rename("p");
        elements_to_score.push(node.clone());
    }
}

fn has_single_tag_inside_element(node: &Node, tag: &str) -> bool {
    // There should be exactly 1 element child with given tag
    let children = node.children();
    if children.len() != 1 {
        return false;
    }

    let first_child = children.first().unwrap();

    if !first_child
        .node_name()
        .map_or(false, |name| name.as_ref() == tag)
    {
        return false;
    }

    !first_child
        .children()
        .iter()
        .any(|n| n.is_text() && RX_HAS_CONTENT.is_match(n.text().as_ref()))
}

fn link_density(node: &Node) -> f32 {
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

fn has_child_block_element(node: &Node) -> bool {
    //TODO: try to improve this! Matcher.match_element()
    node.children().iter().any(|n| {
        if let Some(name) = n.node_name() {
            BLOCK_ELEMS.contains(&name.as_ref()) || has_child_block_element(n)
        } else {
            false
        }
    })
}

fn handle_candidates<'a>(elements_to_score: &mut Vec<NodeRef<'a, NodeData>>, doc: &'a Document) {
    let mut candidates = vec![];

    for element in elements_to_score {
        if !element.is_element() || element.parent().is_none() {
            continue;
        }
        let inner_text = element.text();
        if inner_text.len() < 25 {
            continue;
        }
        let ancestors = element.ancestors(Some(5));

        if ancestors.len() == 0 {
            continue;
        }

        let mut content_score: usize = 1;

        content_score += RX_COMMAS.captures_iter(&inner_text.as_ref()).count();

        content_score += std::cmp::min(inner_text.len() / 100, 3);

        for (level,ancestor) in ancestors.iter().enumerate() {
            if !ancestor.is_element() || ancestor.parent().is_none() {
                continue;

            }

            let score_divider: f32 = match level{
                0 => 1.0,
                1 => 2.0,
                _ => (level * 3) as f32,
            };

            let mut was_initialized = false;

            if !has_node_score(ancestor) {
                init_node_score(ancestor);
                was_initialized = true;
            }

            let mut ancestor_score = get_node_score(ancestor).unwrap();
            ancestor_score += content_score as f32 / score_divider;
            set_node_score(ancestor, ancestor_score);
            
            if was_initialized {
                candidates.push(ancestor.clone());
            }
        }   

    }

    //TODO: this is a crap

    // Scale the final candidates score based on link density. Good content
    // should have a relatively small link density (5% or less) and be mostly
    // unaffected by this operation.
    for candidate in candidates.iter() {
        let prev_score = get_node_score(candidate).unwrap();
        let score = prev_score * (1.0 - link_density(candidate));
        set_node_score(candidate, score);
    }
    candidates.sort_by(|n1, n2| get_node_score(n2).unwrap().partial_cmp(&get_node_score(n1).unwrap()).unwrap());

    let mut top_candidates = candidates;
    top_candidates.truncate(DEFAULT_N_TOP_CANDIDATES);

    let top_candidate = top_candidates.first();
    let top_candidate_name = top_candidate.map_or(None,|n| n.node_name()).unwrap_or_else(|| StrTendril::new());
    
    let page_sel = doc.select("body");
    let page_node =page_sel.nodes().first().unwrap();
    let mut needed_to_create_top_candidate = false;

    if top_candidate.is_none() || top_candidate_name.as_ref() == "body" {
        needed_to_create_top_candidate = true;
        let tc = doc.tree.new_element("div");

        doc.tree.reparent_children_of(&page_node.id, Some(tc.id));
        page_node.append_child(&tc.id);
    }

}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::readability::Readability;

    #[test]
    fn test_removing_probably_invisible_nodes() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <p hidden>This paragraph should be hidden.</p> 
                 <p aria-hidden="true">This paragraph should be hidden.</p>
                 <p style="display:none">This paragraph should be hidden.</p>
                 <p style="visibility:hidden">This paragraph should be hidden.</p>
                 <p aria-hidden="true" class="mwe-math-fallback-image-inline"></p>
                 <p>This paragraph is visible</p>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        grab_article(&doc, None);

        assert_eq!(2, doc.select("p").length());
    }

    #[test]
    fn test_remove_dialog() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <div id="dialog1" role="dialog" aria-modal="true">
                    <h2>Test dialog<h2>
                    <button id="close1">Close</button>
                 </div>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        assert!(doc.select("#dialog1").exists());

        clean_doc(&doc);
        assert!(!doc.select("#dialog1").exists());
        assert!(!doc.select("#close1").exists());
    }

    #[test]
    fn test_unlikely_roles() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <div id="dialog1" role="dialog">
                    <h2>Test dialog<h2>
                    <button id="close1">Close</button>
                 </div>
                 <nav id="nav1" role="navigation"></nav>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        assert!(doc.select("*[role]").exists());

        clean_doc(&doc);
        assert!(!doc.select("*[role]").exists());
    }

    #[test]
    fn test_remove_empty() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <p>This paragraph is visible</p>
                 <header></header>
                 <section></section>
                 <div></div>
                 <h1></h1>
                 <h2></h2>
                 <h3></h3>
                 <h4></h4>
                 <h5></h5>
                 <h6></h6>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        let sel = doc.select("body > *");
        let count_before = sel.nodes().iter().filter(|n| n.is_element()).count();
        assert_eq!(count_before, 10);
        clean_doc(&doc);

        let sel = doc.select("body > *");
        let count_after = sel.nodes().iter().filter(|n| n.is_element()).count();
        assert_eq!(count_after, 1);
    }

    #[test]
    fn test_consume_byline() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <a class="site-title" rel="author" href="/">Cat's Blog</a>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        // consuming byline during grabbing the article
        grab_article(&doc, None);
        assert!(!doc.select("a").exists())
    }

    #[test]
    fn test_skipping_byline() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <a class="site-title" rel="author" href="/">Cat's Blog</a>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        let mut metadata = MetaData::default();
        metadata.byline = "Cat".to_string();
        // consuming byline during grabbing the article
        grab_article(&doc, Some(metadata));
        assert!(doc.select("a").exists())
    }

    #[test]
    fn test_remove_title_duplicates() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Rust (programming language) - Wikipedia</title></head>
            <body>
                 <h1>Rust (programming language)</h1>
            </body>
        </html>"#;

        let readability = Readability::from(contents);
        let metadata = readability.get_article_metadata(None);

        assert!(readability.doc.select("h1").exists());

        grab_article(&readability.doc, Some(metadata));
        assert!(!readability.doc.select("h1").exists())
    }

    #[test]
    fn test_remove_unlikely_candidates() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <h1>Test</h1>
                 <div class="banner">Some annoying content</div>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        assert!(doc.select("div.banner").exists());

        grab_article(&doc, None);
        assert!(!doc.select("div.banner").exists())
    }
    #[test]
    fn test_skip_ok_maybe_candidates() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                 <h1>Test</h1>
                 <a class="banner">Some annoying content</a>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        assert!(doc.select("a.banner").exists());

        grab_article(&doc, None);
        assert!(doc.select("a.banner").exists())
    }
}
