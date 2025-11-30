use dom_query::Tree;
use foldhash::{HashMap, HashSet};
use std::vec;

use dom_query::{Document, NodeId, NodeRef, Selection};
use flagset::FlagSet;
use tendril::StrTendril;

use crate::config::CandidateSelectMode;
#[allow(clippy::wildcard_imports)]
use crate::glob::*;
use crate::grab_flags::GrabFlags;
#[allow(clippy::wildcard_imports)]
use crate::helpers::*;
#[allow(clippy::wildcard_imports)]
use crate::matching::*;
use crate::prep_article::prep_article;
#[allow(clippy::wildcard_imports)]
use crate::score::*;
use crate::Config;
use crate::Metadata;
use crate::Readability;

impl Readability {
    pub(crate) fn grab_article(&self) -> Option<Document> {
        let mut flags: FlagSet<GrabFlags> = FlagSet::full();
        let mut best_attempt: Option<(Document, usize)> = None;
        loop {
            let doc = self.doc.clone();
            let article_node = self.attempt_grab_article(&doc, &flags);
            // Now that we've gone through the full algorithm, check to see if
            // we got any meaningful content. If we didn't, we may need to re-run
            // grabArticle with different flags set. This gives us a higher likelihood of
            // finding the content, and the sieve approach gives us a higher likelihood of
            // finding the -right- content.

            if let Some(ref article_node) = article_node {
                let text_length = article_node.normalized_char_count();
                if text_length >= self.config.char_threshold {
                    return Some(doc);
                }

                if let Some((_, best_text_length)) = best_attempt {
                    if text_length > best_text_length {
                        best_attempt = Some((doc, text_length));
                    }
                } else {
                    best_attempt = Some((doc, text_length));
                }
            }
            if flags.contains(GrabFlags::StripUnlikelys) {
                flags -= GrabFlags::StripUnlikelys;
            } else if flags.contains(GrabFlags::WeightClasses) {
                flags -= GrabFlags::WeightClasses;
            } else if flags.contains(GrabFlags::CleanConditionally) {
                flags -= GrabFlags::CleanConditionally;
            } else {
                // No luck after removing flags,
                // just return the longest text we found during the different loops
                let (best_doc, _) = best_attempt?;
                return Some(best_doc);
            }
        }
    }

    pub(crate) fn attempt_grab_article<'a>(
        &self,
        doc: &'a Document,
        flags: &FlagSet<GrabFlags>,
    ) -> Option<NodeRef<'a>> {
        let selection = doc.select_single("body");
        // html5ever always puts body element, even if it wasn't in the document's contents
        let body_node = selection.nodes().first()?;
        let strip_unlikely = flags.contains(GrabFlags::StripUnlikelys);
        let mut elements_to_score = collect_elements_to_score(body_node, strip_unlikely);
        let article_node = self.handle_candidates(&mut elements_to_score, body_node, flags);
        let res = article_node.map(|n| NodeRef::new(n.id, &doc.tree));
        res
    }

    fn handle_candidates<'a>(
        &self,
        elements_to_score: &mut Vec<NodeRef<'a>>,
        body_node: &'a NodeRef,
        flags: &FlagSet<GrabFlags>,
    ) -> Option<NodeRef<'a>> {
        let tree = body_node.tree;
        let weigh_class = flags.contains(GrabFlags::WeightClasses);
        let top_candidates = score_elements(elements_to_score, tree, &self.config, flags);

        let mut top_candidate = top_candidates.first().copied();

        let mut top_candidate_is_created = false;

        if top_candidate.is_none() || top_candidate.as_ref().is_some_and(|n| n.has_name("body")) {
            top_candidate_is_created = true;
            let tc = tree.new_element("div");

            tree.reparent_children_of(&body_node.id, Some(tc.id));
            body_node.append_child(&tc);
            init_node_score(&tc, flags.contains(GrabFlags::WeightClasses));
            top_candidate = Some(tc);
        } else if top_candidate.is_some() {
            if matches!(
                self.config.candidate_select_mode,
                CandidateSelectMode::DomSmoothie
            ) {
                top_candidate =
                    find_common_candidate_alt(top_candidate, &top_candidates, weigh_class);
            } else {
                // Find a better top candidate node if it contains (at least three) nodes which belong to `topCandidates` array
                // and whose scores are quite closed with current `topCandidate` node.
                top_candidate = find_common_candidate(top_candidate, &top_candidates, weigh_class);
            }
            // If the top candidate is the only child, use parent instead. This will help sibling
            // joining logic when adjacent content is actually located in parent's sibling node.
            if let Some(ref tc) = top_candidate {
                let mut parent_of_top_candidate = tc.parent();

                while let Some(ref tc_parent) = parent_of_top_candidate {
                    if tc_parent.has_name("body") {
                        break;
                    }

                    if tc_parent.element_children().len() != 1 {
                        break;
                    }
                    top_candidate = parent_of_top_candidate;
                    parent_of_top_candidate = tc_parent.parent();
                }
            }
        }
        if let Some(ref tc) = top_candidate {
            if !has_node_score(tc) {
                init_node_score(tc, weigh_class);
            }
            // Now that we have the top candidate, look through its siblings for content
            // that might also be related. Things like preambles, content split by ads
            // that we removed, etc.

            let article_content = tree.new_element("div");

            assign_article_node(tc, &article_content);

            //prepare the article
            prep_article(&article_content, flags, &self.config);

            if top_candidate_is_created {
                tc.set_attr("id", CONTENT_ID);
                tc.set_attr("class", "page");
            } else {
                // this code does the same this as mozilla's implementation, but it is more simpler.
                article_content.set_attr("id", CONTENT_ID);
                article_content.set_attr("class", "page");
            }

            return Some(article_content);
        }
        None
    }
}

pub(crate) fn pre_filter_document(doc: &Document, metadata: &mut Metadata) {
    // Mozilla's implementation performs filtering for each approach of grabbing the article.
    // However, I believe it is better to do it only once. Additionally, there is a lot of logic that relies
    // on a certain element which is going to be removed in the next iteration.
    let body_sel = doc.select_single("body");
    // html5ever always puts body element, even if it wasn't in the document's contents
    let body_node = body_sel.nodes().first().unwrap();
    let mut should_remove_title_header = !metadata.title.is_empty();
    let mut next_node = next_child_or_sibling(body_node, false);
    while let Some(node) = next_node {
        if !is_probably_visible(&node) {
            next_node = next_child_or_sibling(&node, true);
            node.remove_from_parent();
            continue;
        }

        if node.has_name("svg") {
            next_node = next_child_or_sibling(&node, true);
            continue;
        }

        if MATCHER_DIALOGS.match_element(&node) {
            next_node = next_child_or_sibling(&node, true);
            node.remove_from_parent();
            continue;
        }

        if should_remove_title_header
            && MATCHER_HEADING.match_element(&node)
            && text_similarity(&metadata.title, &node.text()) > 0.75
        {
            should_remove_title_header = false;
            next_node = next_child_or_sibling(&node, true);
            node.remove_from_parent();
            continue;
        }

        if metadata.byline.is_none() && is_valid_byline(&node) {
            let byline = if let Some(item_prop_name) = Selection::from(node)
                .select_single("[itemprop=name]")
                .nodes()
                .first()
            {
                item_prop_name.text()
            } else {
                node.text()
            };

            metadata.byline = Some(normalize_spaces(&byline));
            next_node = next_child_or_sibling(&node, true);
            node.remove_from_parent();
            continue;
        }
        next_node = next_child_or_sibling(&node, false);
    }
}

fn get_node_matching_string(node: &NodeRef) -> StrTendril {
    let mut buf = StrTendril::new();
    let Some(el) = node.element_ref() else {
        return buf;
    };

    for attr in &el.attrs {
        if !matches!(attr.name.local.as_ref(), "class" | "id") {
            continue;
        }
        buf.push_tendril(&attr.value);
        buf.push_char(' ');
    }

    buf.make_ascii_lowercase();
    buf
}

fn is_valid_byline(node: &NodeRef) -> bool {
    let mut is_byline = MATCHER_BYLINE.match_element(node);
    if !is_byline {
        let match_string = get_node_matching_string(node);
        is_byline = BYLINE_PATTERNS.iter().any(|p| match_string.contains(p));
    }
    if !is_byline {
        return false;
    }
    let byline_len = node.text().trim().chars().count();
    byline_len > 0 && byline_len < 100
}

fn is_unlikely_candidate(node: &NodeRef) -> bool {
    // Assuming that `<body>` node can't can't reach this function
    if node.has_name("a") {
        return false;
    }

    let match_string = get_node_matching_string(node);
    if match_string.is_empty() {
        return false;
    }

    if !match_unlikely(&match_string) {
        return false;
    }

    !has_ancestor(node, Some(0), |n| {
        let Some(qual_name) = n.qual_name_ref() else {
            return false;
        };
        matches!(qual_name.local.as_ref(), "table" | "code")
    })
}

fn div_into_p(node: &NodeRef) {
    // Turn all divs that don't have children block level elements into p's
    let tree = node.tree;
    // Put phrasing content into paragraphs.
    let mut p_node: Option<NodeRef> = None;
    let mut child_node = node.first_child();
    while let Some(ref child) = child_node {
        let next_sibling = child.next_sibling();
        if is_phrasing_content(child) {
            if let Some(ref p) = p_node {
                p.append_child(child);
            } else if !is_whitespace(child) {
                let raw_p = tree.new_element("p");
                child.insert_before(&raw_p);
                raw_p.append_child(child);
                p_node = Some(raw_p);
            }
        } else if let Some(ref p) = p_node {
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
}

fn has_child_block_element(node: &NodeRef) -> bool {
    node.descendants_it().any(|n| {
        n.element_ref()
            .is_some_and(|el| BLOCK_ELEMS.contains(&el.name.local))
    })
}

fn score_elements<'a>(
    elements_to_score: &Vec<NodeRef<'a>>,
    tree: &'a Tree,
    cfg: &Config,
    flags: &FlagSet<GrabFlags>,
) -> Vec<NodeRef<'a>> {
    let mut score_map: HashMap<NodeId, f32> = HashMap::default();
    let mut cc_cache = CharCounterCache::default();

    for element in elements_to_score {
        if element.parent().is_none() {
            continue;
        }
        let content_len = cc_cache.char_count(element);
        if content_len < 25 {
            continue;
        }
        let ancestors = element.ancestors(Some(5));

        if ancestors.is_empty() {
            continue;
        }

        // Count commas in the element's text content without allocating a new StrTendril.
        // Equivalent to `1 + element.text().split(COMMAS).count()`, but more efficient.
        let mut content_score = 2 + score_text_content(element);
        content_score += std::cmp::min(content_len / 100, 3);
        for (level, ancestor) in ancestors.iter().enumerate() {
            if !ancestor.is_element() || ancestor.parent().is_none() {
                continue;
            }

            let score_divider: f32 = match level {
                0 => 1.0,
                1 => 2.0,
                _ => (level * 3) as f32,
            };

            let mut ancestor_score = if let Some(score) = score_map.get(&ancestor.id) {
                *score
            } else {
                score_map.insert(ancestor.id, 0.0);
                determine_node_score(ancestor, flags.contains(GrabFlags::WeightClasses))
            };

            ancestor_score += content_score as f32 / score_divider;
            score_map
                .entry(ancestor.id)
                .and_modify(|s| *s = ancestor_score);

            if ancestor.has_name("body") {
                break;
            }
        }
    }

    // Scale the final candidates score based on link density. Good content
    // should have a relatively small link density (5% or less) and be mostly
    // unaffected by this operation.

    let mut scored_candidates: Vec<_> = score_map
        .into_iter()
        .filter(|(_, score)| *score > 0.0)
        .map(|(node_id, prev_score)| {
            let candidate = NodeRef::new(node_id, tree);
            // Skipping adjustment of low score
            let score = if prev_score > cfg.min_score_to_adjust {
                prev_score * (1.0 - link_density_fn(&candidate, None, |n| cc_cache.char_count(n)))
            } else {
                prev_score
            };
            set_node_score(&candidate, score);
            (candidate, score)
        })
        .collect();

    scored_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    scored_candidates
        .into_iter()
        .take(cfg.n_top_candidates)
        .map(move |c| c.0)
        .collect()
}

fn assign_article_node(tc: &NodeRef, article_content: &NodeRef) {
    let tc_node_score = get_node_score(tc);
    let mut sibling_score_threshold = tc_node_score * 0.2;
    if sibling_score_threshold < 10.0 {
        sibling_score_threshold = 10.0;
    }
    // Keep potential top candidate's parent node to try to get text direction of it later.
    let Some(tc_parent) = tc.parent() else {
        unreachable!()
    };

    let tc_class = tc.attr_or("class", "");
    let siblings: Vec<NodeRef> = tc_parent.element_children();
    for sibling in &siblings {
        let mut append = false;
        if sibling.id == tc.id {
            append = true;
        } else {
            let mut content_bonus: f32 = 0.0;
            let sibling_class = sibling.attr_or("class", "");
            if !tc_class.is_empty() && sibling_class == tc_class {
                content_bonus += tc_node_score * 0.2;
            }
            let sibling_score = get_node_score(sibling);
            if sibling_score > 0.0 {
                if sibling_score + content_bonus >= sibling_score_threshold {
                    append = true;
                }
            } else if sibling.has_name("p") {
                let sibling_text = sibling.text();
                let node_content = normalize_spaces(&sibling_text);
                let node_length = node_content.chars().count();
                let link_density = link_density(sibling, Some(node_length));

                if (node_length > 80 && link_density < 0.25)
                    || node_length < 80
                        && node_length > 0
                        && link_density == 0.0
                        && is_sentence(&node_content)
                {
                    append = true;
                }
            }
        }

        //appending sibling
        if append {
            if !node_name_in(sibling, &ALTER_TO_DIV_EXCEPTIONS) {
                // We have a node that isn't a common block level element, like a form or td tag.
                // Turn it into a div so it doesn't get filtered out later by accident.
                sibling.rename("div");
            }
            article_content.append_child(&sibling.id);
        }
    }
    tc_parent.append_child(article_content);
}

/// Find a better top candidate across other candidates in a way that `mozilla/readability` does.
fn find_common_candidate<'a>(
    mut top_candidate: Option<NodeRef<'a>>,
    top_candidates: &[NodeRef<'a>],
    weigh_class: bool,
) -> Option<NodeRef<'a>> {
    let Some(ref tc) = top_candidate else {
        return top_candidate;
    };
    let tc_score = get_node_score(tc);

    let mut alternative_candidate_ancestors = vec![];
    for alt in top_candidates.iter().skip(1) {
        if get_node_score(alt) / tc_score >= 0.75 {
            alternative_candidate_ancestors.push(alt.ancestors(Some(0)));
        }
    }
    // MIN_COMMON_ANCESTORS (in mozilla/readability.js -- MINIMUM_TOPCANDIDATES)
    // represents the number of top candidates' ancestors that may be common.
    // The idea is good, but this magic number doesn't always work very well.
    // For example, imagine we have only two candidates, and both are significant.
    // So, we end up with one top candidate and another candidate.
    // However, the second candidate will be excluded in the end because we require
    // at least three (!) lists of ancestors,
    // which is impossible to derive from just one candidate.
    // To adjust the top candidate to share a common ancestor with other candidates,
    // we would need at least three other candidates.
    // Currently, I consider this approach to be flawed...

    if alternative_candidate_ancestors.len() > MIN_COMMON_ANCESTORS {
        let mut parent_of_top_candidate = tc.parent();
        while let Some(ref tc_parent) = parent_of_top_candidate {
            if tc_parent.has_name("body") {
                break;
            }

            let mut lists_containing_this_ancestor = 0;

            for alt_ancestor in &alternative_candidate_ancestors {
                if alt_ancestor.iter().any(|n| n.id == tc_parent.id) {
                    lists_containing_this_ancestor += 1;
                }
            }

            if lists_containing_this_ancestor >= MIN_COMMON_ANCESTORS {
                top_candidate = parent_of_top_candidate;
                break;
            }

            parent_of_top_candidate = tc_parent.parent();
        }
    }

    top_candidate = adjust_top_candidate_by_parent(top_candidate, weigh_class);

    top_candidate
}

/// Find a better top candidate across other candidates (alternative approach).
fn find_common_candidate_alt<'a>(
    mut top_candidate: Option<NodeRef<'a>>,
    top_candidates: &Vec<NodeRef<'a>>,
    weigh_class: bool,
) -> Option<NodeRef<'a>> {
    let Some(ref tc) = top_candidate else {
        return top_candidate;
    };

    if top_candidates.len() < 2 {
        return top_candidate;
    }

    let tc_ancestors = get_node_ancestors_set(tc);
    let tc_score = get_node_score(tc);

    let mut ancestor_match_counter: HashMap<NodeId, usize> = HashMap::default();

    for alt in top_candidates.iter().skip(1) {
        if get_node_score(alt) / tc_score >= 0.75 {
            let alt_ancestors = get_node_ancestors_set(alt);
            if alt_ancestors.contains(&tc.id) {
                continue;
            }
            let intersect = tc_ancestors.intersection(&alt_ancestors);
            for item in intersect {
                *ancestor_match_counter.entry(*item).or_insert(0) += 1;
            }
        }
    }

    let mut require_adjustment = true;
    // choosing the best candidate by how close it to the top candidate,
    // and then by how many common ancestors it has across all other candidates
    if let Some(best_candidate_id) = ancestor_match_counter
        .into_iter()
        .max_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)))
        .map(|n| n.0)
    {
        let threshold = get_node_score(tc) / 3.0;
        let best_candidate = NodeRef::new(best_candidate_id, tc.tree);
        if get_node_score(&best_candidate) > threshold {
            top_candidate = Some(best_candidate);
            require_adjustment = false;
        }
    }

    if require_adjustment {
        top_candidate = adjust_top_candidate_by_parent(top_candidate, weigh_class);
    }
    top_candidate
}

fn get_node_ancestors_set(node: &NodeRef) -> HashSet<NodeId> {
    // only elements, no html or body, and have a score
    node.ancestors(Some(0))
        .iter()
        .filter(|n| {
            n.is_element()
                && !matches!(n.node_name().as_deref(), Some("html" | "body"))
                && has_node_score(n)
        })
        .map(|n| n.id)
        .collect::<HashSet<_>>()
}

fn adjust_top_candidate_by_parent(
    mut top_candidate: Option<NodeRef<'_>>,
    weigh_class: bool,
) -> Option<NodeRef<'_>> {
    if let Some(ref tc) = top_candidate {
        if !has_node_score(tc) {
            init_node_score(tc, weigh_class);
        }
        // Because of our bonus system, parents of candidates might have scores
        // themselves. They get half of the node. There won't be nodes with higher
        // scores than our topCandidate, but if we see the score going *up* in the first
        // few steps up the tree, that's a decent sign that there might be more content
        // lurking in other places that we want to unify in. The sibling stuff
        // below does some of that - but only if we've looked high enough up the DOM
        // tree.
        let mut last_score = get_node_score(tc);
        let score_threshold = last_score / 3.0;
        let mut parent_of_top_candidate = tc.parent();
        while let Some(ref tc_parent) = parent_of_top_candidate {
            if tc_parent.has_name("body") {
                break;
            }

            if !has_node_score(tc_parent) {
                parent_of_top_candidate = tc_parent.parent();
                continue;
            }

            let parent_score = get_node_score(tc_parent);
            if parent_score < score_threshold {
                break;
            }
            if parent_score > last_score {
                top_candidate = parent_of_top_candidate;
                break;
            }
            last_score = parent_score;
            parent_of_top_candidate = tc_parent.parent();
        }
    }
    top_candidate
}

fn next_child_or_sibling<'a>(node: &NodeRef<'a>, ignore_child: bool) -> Option<NodeRef<'a>> {
    if !ignore_child {
        if let Some(first_child) = node.first_element_child() {
            return Some(first_child);
        }
    }

    if let Some(sibling) = node.next_element_sibling() {
        return Some(sibling);
    }
    let mut parent = node.parent();
    while let Some(parent_node) = parent {
        if let Some(next_sibling) = parent_node.next_element_sibling() {
            return Some(next_sibling);
        }
        parent = parent_node.parent();
        
    }
    None
}

fn collect_elements_to_score<'a>(root_node: &'a NodeRef, strip_unlikely: bool) -> Vec<NodeRef<'a>> {
    let tree = &root_node.tree;
    let mut elements_id_to_score: Vec<NodeId> = vec![];
    let mut next_node = next_child_or_sibling(root_node, false);
    while let Some(mut node) = next_node {
        if node.has_name("svg") {
            next_node = next_child_or_sibling(&node, true);
            continue;
        }

        if strip_unlikely {
            let strip = is_unlikely_candidate(&node)
                || node
                    .attr("role")
                    .is_some_and(|role| UNLIKELY_ROLES.contains(&role));
            if strip {
                next_node = next_child_or_sibling(&node, true);
                node.remove_from_parent();
                continue;
            }
        }
        if node_name_in(&node, &TAGS_WITH_CONTENT) && is_element_without_content(&node) {
            next_node = next_child_or_sibling(&node, true);
            node.remove_from_parent();
            continue;
        }

        if node_name_in(&node, &DEFAULT_TAGS_TO_SCORE) {
            elements_id_to_score.push(node.id);
        }

        if node.has_name("div") {
            div_into_p(&node);

            // Sites like http://mobile.slate.com encloses each paragraph with a DIV
            // element. DIVs with only a P element inside and no text content can be
            // safely converted into plain P elements to avoid confusing the scoring
            // algorithm with DIVs with are, in practice, paragraphs.

            if has_single_tag_inside_element(&node, "p") && link_density(&node, None) < 0.25 {
                let new_node = node.first_element_child().unwrap();
                node.replace_with(&new_node);
                elements_id_to_score.push(new_node.id);
                node = new_node;
            } else if !has_child_block_element(&node) {
                node.rename("p");
                elements_id_to_score.push(node.id);
            }
        }
        next_node = next_child_or_sibling(&node, false);
    }
    elements_id_to_score
        .iter()
        .map(|n| NodeRef::new(*n, tree))
        .collect()
}

#[cfg(not(feature = "aho-corasick"))]
fn match_unlikely(haystack: &str) -> bool {
    let check = BytePatternCheck::new(haystack);

    if !check.contains_any(UNLIKELY_CANDIDATES) {
        return false;
    }
    if check.contains_any(MAYBE_CANDIDATES) {
        return false;
    }
    true
}

#[cfg(feature = "aho-corasick")]
fn match_unlikely(haystack: &str) -> bool {
    if !crate::ac_automat::AC_UNLIKELY.is_match(haystack) {
        return false;
    }
    if crate::ac_automat::AC_MAYBE.is_match(haystack) {
        return false;
    }
    true
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
                 <p style="display: none !important">This paragraph should be hidden.</p>
                 <p style="display: none!important">This paragraph should be visible.</p>
                 <p style="display:">This paragraph should be visible.</p>
                 <p style="display">This paragraph should be visible.</p>
                 <p style=":">This paragraph should be visible.</p>
                 <p style="visibility:hidden">This paragraph should be hidden.</p>
                 <p aria-hidden="true" class="mwe-math-fallback-image-inline">123*123</p>
                 <p>This paragraph is visible</p>
                 <p style="DISPLAY: NONE">This paragraph should be hidden.</p>
                 <p style="display: none; visibility: visible">This paragraph should be hidden.</p>
                 <p style="font-family: 'Times New Roman'; display: none">This paragraph should be hidden.</p>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        let mut meta = Metadata::default();
        pre_filter_document(&doc, &mut meta);

        assert_eq!(6, doc.select("p").length());
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

        pre_filter_document(&doc, &mut Metadata::default());
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

        collect_elements_to_score(&doc.root(), true);
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

        let ra = Readability::new(contents, None, None).unwrap();
        let sel = ra.doc.select("body > *");
        let count_before = sel.nodes().iter().filter(|n| n.is_element()).count();

        assert_eq!(count_before, 10);

        let clean_doc = ra.grab_article().unwrap();
        let sel = clean_doc.select("body > *");
        let count_after = sel.nodes().iter().filter(|n| n.is_element()).count();
        assert_eq!(count_after, 1);
    }

    #[test]
    fn test_consume_byline() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                <div>
                 <a class="site-title" rel="author" href="/">Cat's Blog</a>
                <p>Content</p>
                 </div>
            </body>
        </html>"#;

        let doc = Document::from(contents);
        // consuming byline during grabbing the article
        pre_filter_document(&doc, &mut Metadata::default());
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
        let mut metadata = Metadata {
            byline: Some("Cat".to_string()),
            ..Default::default()
        };
        // consuming byline during grabbing the article
        pre_filter_document(&doc, &mut metadata);
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
        let mut metadata = readability.get_article_metadata(None);

        assert!(readability.doc.select("h1").exists());
        pre_filter_document(&readability.doc, &mut metadata);

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

        collect_elements_to_score(&doc.root(), true);
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
        collect_elements_to_score(&doc.root(), true);
        assert!(doc.select("a.banner").exists())
    }
}
