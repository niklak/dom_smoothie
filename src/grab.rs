use std::cmp::Reverse;
use std::vec;

use dom_query::{Document, Node, NodeRef};
use flagset::FlagSet;
use tendril::StrTendril;

use crate::glob::*;
use crate::grab_flags::GrabFlags;
use crate::score::*;

use crate::helpers::*;
use crate::prep_article::prep_article;
use crate::MetaData;
//TODO: do not forget FLAGS

pub fn grab_article(doc: &Document, metadata: Option<MetaData>) -> Option<Document> {
    let mut metadata = metadata.unwrap_or_default();

    clean_doc(doc);



    if !metadata.title.is_empty() {
        // if title is not empty then delete duplicate
        remove_header_duplicates_title(doc, &metadata.title);
    }

    let mut flags =
        GrabFlags::CleanConditionally | GrabFlags::StripUnlikelys | GrabFlags::WeightClasses;

    let mut attempts = vec![];

    loop {
        let mut elements_to_score: Vec<NodeRef<'_>> = vec![];
        let doc = doc.clone();
        let selection = doc.select("*");
        //TODO: maybe this way of iterating through nodes is not the best
        for node in selection.nodes().iter().filter(|n| n.is_element()) {
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

            if flags.contains(GrabFlags::StripUnlikelys)
                && is_unlikely_candidate(node, &match_string)
            {
                node.remove_from_parent();
                continue;
            }

            let node_name = node.node_name().unwrap();

            if TAGS_WITH_CONTENT.contains(&node_name.as_ref()){
                if is_element_without_content(node) {
                    node.remove_from_parent();
                    continue;
                } 
            }

            if DEFAULT_TAGS_TO_SCORE.contains(&node_name.as_ref()) {
                elements_to_score.push(node.clone());
            }

            // TODO: div_matcher.match_element(node)

            if node_name.as_ref() == "div" {
                div_into_p(node, &doc, &mut elements_to_score);
            }
        }
        
        let article_node = handle_candidates(&mut elements_to_score, &doc, &flags);
        let mut parse_successful = true;

        let mut article_doc: Option<Document> = None;

        if let Some(ref article_node) = article_node {
            article_doc = Some(Document::from(article_node.html()));
            let text_length = normalize_spaces(&article_node.text()).chars().count();
            if text_length < DEFAULT_CHAR_THRESHOLD {
                parse_successful = false;

                attempts.push((article_doc.clone(), text_length));
                if flags.contains(GrabFlags::StripUnlikelys) {
                    flags -= GrabFlags::StripUnlikelys;
                } else if flags.contains(GrabFlags::WeightClasses) {
                    flags -= GrabFlags::WeightClasses;
                } else if flags.contains(GrabFlags::CleanConditionally) {
                    flags -= GrabFlags::CleanConditionally;
                } else {
                    // No luck after removing flags, just return the longest text we found during the different loops
                    attempts.sort_by_key(|i| Reverse(i.1));

                    if attempts[0].1 == 0 {
                        return None;
                    }
                    article_doc = attempts[0].0.clone();
                    parse_successful = true;
                }
            }
        } else {
            parse_successful = false;
        }

        if parse_successful {
            return article_doc;
        }
        // Now that we've gone through the full algorithm, check to see if
        // we got any meaningful content. If we didn't, we may need to re-run
        // grabArticle with different flags set. This gives us a higher likelihood of
        // finding the content, and the sieve approach gives us a higher likelihood of
        // finding the -right- content.
    }
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

fn get_node_matching_string(node: &NodeRef) -> String {
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
    if has_ancestor_tag::<NodePredicate>(node, "table", Some(0), None) {
        return false;
    }
    if has_ancestor_tag::<NodePredicate>(node, "code", Some(0), None) {
        return false;
    }
    true
}

fn div_into_p<'a>(node: &'a Node, doc: &'a Document, elements_to_score: &mut Vec<NodeRef<'a>>) {
    // Turn all divs that don't have children block level elements into p's

    // Put phrasing content into paragraphs.
    let mut p_node: Option<Node> = None;
    let mut child_node = node.first_child();
    while let Some(ref child) = child_node {
        let next_sibling = child.next_sibling();
        if is_phrasing_content(child) {

            if let Some(ref p) = p_node {
                p.append_child(child);
            } else if !is_whitespace(child) {
                let raw_p = doc.tree.new_element("p");
                child.insert_before(&raw_p);
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
        child_node = next_sibling;
    }

    // Sites like http://mobile.slate.com encloses each paragraph with a DIV
    // element. DIVs with only a P element inside and no text content can be
    // safely converted into plain P elements to avoid confusing the scoring
    // algorithm with DIVs with are, in practice, paragraphs.

    if has_single_tag_inside_element(node, "p") && link_density(node) < 0.25 {
        let new_node = node.first_element_child().unwrap();
        node.replace_with(&new_node);
        elements_to_score.push(new_node.clone());
    } else if !has_child_block_element(node) {
        node.rename("p");
        elements_to_score.push(node.clone());
    }
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

fn handle_candidates<'a>(
    elements_to_score: &mut Vec<NodeRef<'a>>,
    doc: &'a Document,
    flags: &FlagSet<GrabFlags>,
) -> Option<NodeRef<'a>> {
    let mut candidates = vec![];
    let mut visited = vec![];

    for element in elements_to_score {

        // TODO: made it without duplicates!

        if visited.contains(&element.id) {
            continue;
        }

        visited.push(element.id);

        if !element.is_element() || element.parent().is_none() {
            continue;
        }
        let inner_text = normalize_spaces(&element.text());
        if inner_text.chars().count() < 25 {
            continue;
        }
        let ancestors = element.ancestors(Some(5));

        if ancestors.is_empty() {
            continue;
        }

        let mut content_score: usize = 1;

        content_score += RX_COMMAS.split(inner_text.as_ref()).count();

        content_score += std::cmp::min(inner_text.chars().count() / 100, 3);
        for (level, ancestor) in ancestors.iter().enumerate() {
            if !ancestor.is_element() || ancestor.parent().is_none() {
                continue;
            }

            let score_divider: f32 = match level {
                0 => 1.0,
                1 => 2.0,
                _ => (level * 3) as f32,
            };

            if !has_node_score(ancestor) {
                init_node_score(ancestor, flags.contains(GrabFlags::WeightClasses));
                candidates.push(ancestor.clone());
            }

            let mut ancestor_score = get_node_score(ancestor).unwrap();
            ancestor_score += content_score as f32 / score_divider;
            set_node_score(ancestor, ancestor_score);

        }
    }

    // Scale the final candidates score based on link density. Good content
    // should have a relatively small link density (5% or less) and be mostly
    // unaffected by this operation.


    for candidate in candidates.iter() {
        let prev_score = get_node_score(candidate).unwrap();
        let score = prev_score * (1.0 - link_density(candidate));
        set_node_score(candidate, score);
    }
    
    

    candidates.sort_by(|n1, n2| {
        get_node_score(n2)
            .unwrap()
            .partial_cmp(&get_node_score(n1).unwrap())
            .unwrap()
    });


    let mut top_candidates = candidates;
    top_candidates.truncate(DEFAULT_N_TOP_CANDIDATES);


    // TODO: revise everything below till line 460

    let mut top_candidate = top_candidates.first().cloned();

    let top_candidate_name = top_candidate
        .clone()
        .and_then(|ref n| n.node_name())
        .unwrap_or_else(StrTendril::new);

    let page_sel = doc.select("body");
    let page_node = page_sel.nodes().first().unwrap();
    let mut needed_to_create_top_candidate = false;

    if top_candidate.is_none() || top_candidate_name.as_ref() == "body" {
        needed_to_create_top_candidate = true;

        let tc = doc.tree.new_element("div");

        doc.tree.reparent_children_of(&page_node.id, Some(tc.id));
        page_node.append_child(&tc.id);
        init_node_score(&tc, flags.contains(GrabFlags::WeightClasses));
        top_candidate = Some(tc.clone());
    } else if let Some(ref tc) = top_candidate {
        // Find a better top candidate node if it contains (at least three) nodes which belong to `topCandidates` array
        // and whose scores are quite closed with current `topCandidate` node.
        // TODO: this isn't working

        let tc_score = get_node_score(tc).unwrap();

        let mut alternative_candidate_ancestors = vec![];
        for alt in top_candidates.iter().skip(1) {
            if get_node_score(alt).unwrap() / tc_score >= 0.75 {
                alternative_candidate_ancestors.push(alt.ancestors(Some(0)));
            }
        }
        if alternative_candidate_ancestors.len() > MINIMUM_TOP_CANDIDATES {
            let mut parent_of_top_candidate = tc.parent();
            while let Some(ref parent_of_tc) = parent_of_top_candidate {
                if let Some(node_name) = parent_of_tc.node_name() {
                    if node_name.as_ref() == "body" {
                        break;
                    }
                }
                
                let mut lists_containing_this_ancestor = 0;

                for alt_ancestor in &alternative_candidate_ancestors {
                    if lists_containing_this_ancestor >= MINIMUM_TOP_CANDIDATES {
                        break;
                    }

                    if alt_ancestor.iter().any(|n| n.id == parent_of_tc.id) {
                        lists_containing_this_ancestor += 1;
                    }
                }

                if lists_containing_this_ancestor >= MINIMUM_TOP_CANDIDATES {
                    top_candidate = parent_of_top_candidate;
                    break;
                }

                parent_of_top_candidate = parent_of_tc.parent();
            }
        }

        if let Some(ref tc) = top_candidate {
            if !has_node_score(tc) {
                init_node_score(tc, flags.contains(GrabFlags::WeightClasses));
            }
            // Because of our bonus system, parents of candidates might have scores
            // themselves. They get half of the node. There won't be nodes with higher
            // scores than our topCandidate, but if we see the score going *up* in the first
            // few steps up the tree, that's a decent sign that there might be more content
            // lurking in other places that we want to unify in. The sibling stuff
            // below does some of that - but only if we've looked high enough up the DOM
            // tree.
            let mut last_score = get_node_score(tc).unwrap();
            let score_threshold = last_score / 3.0;
            let mut parent_of_top_candidate = tc.parent();
            while let Some(ref parent_of_tc) = parent_of_top_candidate {
                let node_name = parent_of_tc.node_name().unwrap();
                if node_name.as_ref() == "body" {
                    break;
                }
                if !has_node_score(parent_of_tc) {
                    parent_of_top_candidate = parent_of_tc.parent();
                    continue;
                }

                let parent_score = get_node_score(parent_of_tc).unwrap();
                if parent_score < score_threshold {
                    break;
                }
                if parent_score > last_score {
                    top_candidate = parent_of_top_candidate;
                    break;
                }
                last_score = parent_score;
                parent_of_top_candidate = parent_of_tc.parent();
            }
        }

        // If the top candidate is the only child, use parent instead. This will help sibling
        // joining logic when adjacent content is actually located in parent's sibling node.
        if let Some(ref tc) = top_candidate {
            let mut parent_of_top_candidate = tc.parent();

            while let Some(ref parent_of_tc) = parent_of_top_candidate {
                let node_name = parent_of_tc.node_name().unwrap();

                if node_name.as_ref() == "body" {
                    break;
                }

                if parent_of_tc.element_children().len() != 1 {
                    break;
                }
                top_candidate = parent_of_top_candidate.clone();
                parent_of_top_candidate = parent_of_tc.parent();
            }
        }
    }
    if let Some(ref tc) = top_candidate {
        if !has_node_score(tc) {
            init_node_score(tc, flags.contains(GrabFlags::WeightClasses));
        }

        // Now that we have the top candidate, look through its siblings for content
        // that might also be related. Things like preambles, content split by ads
        // that we removed, etc.

        let mut article_content = doc.tree.new_element("div");
        article_content.set_attr("id", "readability-content");

        handle_top_candidate(tc, &article_content);

        //prepare the article
        prep_article(&article_content, flags);

        if needed_to_create_top_candidate {
            // This looks like nonsense
            // We already created a fake div thing, and there wouldn't have been any siblings left
            // for the previous loop, so there's no point trying to create a new div, and then
            // move all the children over. Just assign IDs and class names here. No need to append
            // because that already happened anyway.
            article_content.set_attr("id", "readability-page-1");
            article_content.set_attr("class", "page");
        } else {
            let div = doc.tree.new_element("div");
            div.set_attr("id", "readability-page-1");
            div.set_attr("class", "page");
            doc.tree
                .reparent_children_of(&article_content.id, Some(div.id));
            article_content.replace_with(&div);
            article_content = div;
        }

        set_dir_attr(&article_content);

        return Some(article_content);
    }
    None
}

fn handle_top_candidate(tc: &Node, article_content: &Node) {
    let tc_node_score = get_node_score(tc).unwrap();
    let mut sibling_score_threshold = tc_node_score * 0.2;
    if sibling_score_threshold < 10.0 {
        sibling_score_threshold = 10.0;
    }
    // Keep potential top candidate's parent node to try to get text direction of it later.
    let parent_of_top_candidate = tc.parent().unwrap();

    let mut siblings: Vec<Node> = parent_of_top_candidate.element_children();

    let mut s = 0;

    while s < siblings.len() {
        let sibling = siblings.get(s).unwrap();
        let sibling_name = sibling.node_name().unwrap();
        let mut append = false;
        if sibling.id == tc.id {
            append = true;
        } else {
            let mut content_bonus: f32 = 0.0;
            let sibling_class = sibling.attr_or("class", "");
            let tc_class = tc.attr_or("class", "");
            if !tc_class.is_empty() && sibling_class == tc_class {
                content_bonus += tc_node_score * 0.2;
            }

            if let Some(sibling_score) = get_node_score(sibling) {
                if sibling_score + content_bonus >= sibling_score_threshold {
                    append = true;
                }
            } else if sibling_name.as_ref() == "p" {
                let link_density = link_density(sibling);
                let node_content = normalize_spaces(&sibling.text());
                let node_length = node_content.chars().count();

                if (node_length > 80 && link_density < 0.25)
                    || node_length < 80
                        && node_length > 0
                        && link_density == 0.0
                        && RX_SENTENCE.is_match(&node_content)
                {
                    append = true;
                }
            }
        }

        //appending sibling
        if append {
            if !ALTER_TO_DIV_EXCEPTIONS.contains(&sibling_name.as_ref()) {
                // We have a node that isn't a common block level element, like a form or td tag.
                // Turn it into a div so it doesn't get filtered out later by accident.
                sibling.rename("div");
            }

            sibling.remove_from_parent();
            article_content.append_child(&sibling.id);

            siblings = parent_of_top_candidate.element_children();

            s = s.saturating_sub(1);
        }
        s += 1;
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
