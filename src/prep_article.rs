use dom_query::{Node, Selection};
use flagset::FlagSet;
use tendril::format_tendril;

use crate::glob::*;
use crate::grab_flags::GrabFlags;
use crate::helpers::*;
use crate::score::*;
use crate::Config;

fn clean(n: &Node, tag: &str) {
    let is_embed = EMBED_ELEMENTS.contains(tag);

    for node in n.find(&[tag]) {
        // Allow youtube and vimeo videos through as people usually want to see those.
        let mut should_remove = true;
        if is_embed {
            for attr in node.attrs().iter() {
                if RX_VIDEO_ATTRS.is_match(&attr.value) {
                    should_remove = false;
                    break;
                }
            }

            if node
                .node_name()
                .map_or(false, |name| name.as_ref() == "embed")
                && RX_VIDEO_ATTRS.is_match(&node.inner_html())
            {
                should_remove = false;
            }
        }

        if should_remove {
            node.remove_from_parent();
        }
    }
}

fn clean_styles(n: &Node) {
    if !n.is_element() {
        return;
    }

    let Some(node_name) = n.node_name() else {
        return;
    };
    if node_name.as_ref() == "svg" {
        return;
    }

    n.remove_attrs(PRESENTATIONAL_ATTRIBUTES);

    if DEPRECATED_SIZE_ATTRIBUTE_ELEMS.contains(&node_name) {
        n.remove_attrs(&["width", "height"]);
    }

    for child_node in n.element_children().iter() {
        clean_styles(child_node);
    }
}

fn should_clean_conditionally(node: &Node, tag: &str, flags: &FlagSet<GrabFlags>) -> bool {
    let sel = Selection::from(node.clone());
    let mut is_list = matches!(tag, "ul" | "ol");
    if !is_list {
        let list_length = sel
            .select("ul, ol")
            .iter()
            .fold(0, |acc, s| acc + s.text().trim().chars().count());
        is_list = (list_length as f32 / sel.text().trim().chars().count() as f32) > 0.9;
    }

    let is_data_table = |n: &Node| n.has_attr("data-readability-table");

    if tag == "table" && is_data_table(node) {
        return false;
    }
    // Next check if we're inside a data table, in which case don't remove it as well.
    if has_ancestor_tag(node, "table", None, Some(is_data_table)) {
        return false;
    }

    if has_ancestor_tag::<NodePredicate>(node, "code", Some(0), None) {
        return false;
    }

    // keep element if it has a data tables
    if sel.select("table[data-readability-table]").exists() {
        return false;
    }

    let weight = get_class_weight(node, flags.contains(GrabFlags::WeightClasses));

    if weight < 0.0 {
        return true;
    }

    if node.text().matches(",").count() < 10 {
        // If there are not very many commas, and the number of
        // non-paragraph elements is more than paragraphs or other
        // ominous signs, remove the element.
        let p: f32 = node.find(&["p"]).len() as f32;
        let img = node.find(&["img"]).len() as f32;
        let li = node.find(&["li"]).len() as f32 - 100.0;
        let input = node.find(&["input"]).len() as f32;
        let heading_density = get_text_density(node, "h1,h2,h3,h4,h5,h6");

        let mut embed_count = 0;

        let embeds_sel = sel.select_matcher(&MATCHER_EMBEDS);

        for embed in embeds_sel.nodes().iter() {
            for attr in embed.attrs().iter() {
                if RX_VIDEO_ATTRS.is_match(&attr.value) {
                    return false;
                }
            }
            if embed
                .node_name()
                .map_or(false, |name| name.as_ref() == "embed")
                && RX_VIDEO_ATTRS.is_match(&embed.inner_html())
            {
                return false;
            }
            embed_count += 1;
        }

        let inner_text = sel.text();
        let trimmed_text = inner_text.trim();
        if RX_AD_WORDS.is_match(trimmed_text) || RX_LOADING_WORDS.is_match(trimmed_text) {
            return true;
        }

        let should_remove = || {
            let is_figure_child = has_ancestor_tag::<NodePredicate>(node, "figure", None, None);

            if !is_figure_child && img > 1.0 && p / img < 0.5 {
                return true;
            }
            if !is_list && li > p {
                return true;
            }
            if input > (p / 3.0).floor() {
                return true;
            }

            let content_length = normalized_char_count(&inner_text);
            let link_density = link_density(node);

            if !is_list
                && !is_figure_child
                && heading_density < 0.9
                && content_length < 25
                && (img == 0.0 || img > 2.0)
                && link_density > 0.0
            {
                return true;
            }
            if !is_list && weight < 25.0 && link_density > 0.2 {
                return true;
            }
    
            if weight >= 25.0 && link_density > 0.5 {
                return true;
            }
    
            if (embed_count == 1 && content_length < 75) || embed_count > 1 {
                return true;
            }

            let text_density = get_text_density(node, &TEXTISH_TAGS.join(","));
            if img == 0.0 && text_density == 0.0 {
                return true;
            }
            false
        };
        let have_to_remove = should_remove();
        
        if is_list && have_to_remove {
            for child in node.children_it(false) {
                if child.element_children().len() > 1 {
                    return have_to_remove;
                }
            }
            let li_count = node.find(&["li"]).len();
            if img == li_count as f32 {
                return false;
            }
        }

        return have_to_remove;
    }
    false
}

fn clean_conditionally(node: &Node, tag: &str, flags: &FlagSet<GrabFlags>) {
    if !flags.contains(GrabFlags::CleanConditionally) {
        return;
    }

    let tag_sel = Selection::from(node.clone()).select(tag);
    // traversing tag nodes in reverse order,
    // so that how children nodes will appear before parent nodes
    for tag_node in tag_sel.nodes().iter().rev() {
        if should_clean_conditionally(tag_node, tag, flags) {
            tag_node.remove_from_parent();
        }
    }
}

fn set_data_readability_table(n: &Node, is_data_table: bool) {
    if is_data_table {
        n.set_attr("data-readability-table", "true");
    } else {
        n.remove_attr("data-readability-table");
    }
}

fn get_row_and_col_count(table: &Selection) -> (usize, usize) {
    let mut rows = 0usize;
    let mut cols = 0usize;
    for tr in table.select("tr").iter() {
        let rowspan = table.attr_or("rowspan", "1");
        rows += rowspan.parse::<usize>().unwrap_or(1);

        //Now look for column-related info
        let mut columns_in_this_row = 0;

        for cell in tr.select("td").iter() {
            let colspan = cell.attr_or("colspan", "1");
            columns_in_this_row += colspan.parse::<usize>().unwrap_or(1);
        }
        cols = std::cmp::max(cols, columns_in_this_row);
    }

    (rows, cols)
}

fn mark_data_tables(n: &Node) {
    // TODO: revise this
    let table_sel = Selection::from(n.clone()).select("table");

    for node in table_sel.nodes().iter() {
        let sel = Selection::from(node.clone());

        let role = node.attr_or("role", "");
        if role.as_ref() == "presentation" {
            set_data_readability_table(node, false);
            continue;
        }

        let data_table = node.attr_or("datatable", "");
        if data_table.as_ref() == "0" {
            set_data_readability_table(node, false);
            continue;
        }

        if node.has_attr("summary") {
            set_data_readability_table(node, true);
            continue;
        }
        let caption_sel = sel.select("caption");
        if caption_sel.exists() {
            set_data_readability_table(node, true);
            continue;
        }

        let descendants_sel = sel.select("col,colgroup,tfoot,thead,th");

        if descendants_sel.exists() {
            set_data_readability_table(node, true);
            continue;
        }

        // nested tables indicate a layout table
        if sel.select("table").exists() {
            set_data_readability_table(node, false);
            continue;
        }

        let (rows, cols) = get_row_and_col_count(&sel);
        if rows == 1 || cols == 1 {
            set_data_readability_table(node, false);
            continue;
        }

        if rows >= 10 || cols > 4 {
            set_data_readability_table(node, true);
            continue;
        }
        set_data_readability_table(node, (rows * cols) > 10);
    }
}

fn fix_lazy_images(n: &Node) {
    for node in Selection::from(n.clone())
        .select("img,picture,figure")
        .nodes()
        .iter()
    {
        let src = node.attr_or("src", "");
        // In some sites (e.g. Kotaku), they put 1px square image as base64 data uri in the src attribute.
        // So, here we check if the data uri is too short, just might as well remove it.
        if !src.is_empty() {
            if let Some(parts) = RX_BASE64_URL.captures(&src) {
                if parts.len() != 2 {
                    continue;
                }
                if &parts[1] == "image/svg+xml" {
                    continue;
                }

                // Make sure this element has other attributes which contains image.
                // If it doesn't, then this src is important and shouldn't be removed.
                let mut src_could_be_removed = false;

                for attr in node.attrs().iter() {
                    if &attr.name.local == "src" {
                        continue;
                    }

                    if RX_IMG_ATTR.is_match(&attr.value) {
                        src_could_be_removed = true;
                        break;
                    }
                }
                // Here we assume if image is less than 100 bytes (or 133 after encoded to base64)
                // it will be too small, therefore it might be placeholder image.
                if src_could_be_removed {
                    let base64_starts = &parts[1].len();
                    let base64_len = src.len() - base64_starts;
                    if base64_len < 133 {
                        node.remove_attr("src");
                    }
                }
            }
        }

        if (node.has_attr("src") || node.has_attr("srcset")) && !node.has_class("lazy") {
            continue;
        }

        for attr in node.attrs().iter() {
            if matches!(attr.name.local.as_ref(), "src" | "srcset" | "alt") {
                continue;
            }

            let mut copy_to: Option<&str> = None;

            if RX_IMG_ATTR_TO_SRCSET.is_match(&attr.value) {
                copy_to = Some("srcset");
            } else if RX_IMG_ATTR_TO_SRC.is_match(&attr.value) {
                copy_to = Some("src");
            }

            if let Some(copy_to) = copy_to {
                //if this is an img or picture, set the attribute directly
                let Some(tag_name) = node.node_name() else {
                    continue;
                };
                if matches!(tag_name.as_ref(), "img" | "picture") {
                    node.set_attr(copy_to, &attr.value);
                } else if tag_name.as_ref() == "figure" {
                    let figure_sel = Selection::from(node.clone());
                    if !figure_sel.select("img, picture").exists() {
                        //if the item is a <figure> that does not contain an image or picture, create one and place it inside the figure
                        //see the nytimes-3 testcase for an example
                        let img_node = node.tree.new_element("img");
                        img_node.set_attr(copy_to, &attr.value);
                        node.append_child(&img_node.id);
                    }
                }
            }
        }
    }
}

fn clean_headers(n: &Node, flags: &FlagSet<GrabFlags>) {
    for h_node in Selection::from(n.clone())
        .select_matcher(&MATCHER_HEADING)
        .nodes()
        .iter()
    {
        if get_class_weight(h_node, flags.contains(GrabFlags::WeightClasses)) < 0.0 {
            h_node.remove_from_parent();
        }
    }
}

pub(crate) fn prep_article(article_node: &Node, flags: &FlagSet<GrabFlags>, cfg: &Config) {
    clean_styles(article_node);

    // Check for data tables before we continue, to avoid removing items in
    // those tables, which will often be isolated even though they're
    // visually linked to other content-ful elements (text, images, etc.).

    mark_data_tables(article_node);
    fix_lazy_images(article_node);

    clean_conditionally(article_node, "form", flags);

    clean_conditionally(article_node, "fieldset", flags);

    // Clean out junk from the article content
    clean(article_node, "object");
    clean(article_node, "embed");
    clean(article_node, "footer");
    clean(article_node, "link");
    clean(article_node, "aside");

    let share_element_threshold = cfg.char_threshold;

    // Clean out elements with little content that have "share" in their id/class combinations from final top candidates,
    // which means we don't remove the top candidates even they have "share".

    for child in article_node.descendants() {
        let class = child.attr_or("class", "");
        let id = child.attr_or("id", "");
        let class_and_id = format_tendril!("{} {}", class, id);
        if RX_SHARE_ELEMENTS.is_match(&class_and_id) && child.text().len() < share_element_threshold
        {
            child.remove_from_parent();
        }
    }

    clean(article_node, "iframe");
    clean(article_node, "input");
    clean(article_node, "textarea");
    clean(article_node, "select");
    clean(article_node, "button");
    clean_headers(article_node, flags);

    // Do these last as the previous stuff may have removed junk
    // that will affect these
    clean_conditionally(article_node, "table", flags);
    clean_conditionally(article_node, "ul", flags);
    clean_conditionally(article_node, "div", flags);

    // replace H1 with H2 as H1 should be only title that is displayed separately

    let article_sel = Selection::from(article_node.clone());

    article_sel.select("h1").rename("h2");

    // Remove extra paragraphs

    // At this point, nasty iframes have been removed; only embedded video
    // ones remain.
    for p_sel in article_sel.select("p").iter() {
        let content_el_count = p_sel.select("img,object,embed,iframe").length();
        let text = p_sel.text();
        if content_el_count == 0 && text.trim().is_empty() {
            p_sel.remove();
        }
    }

    for br_node in article_node.find(&["br"]).iter() {
        if let Some(next_node) = br_node.next_element_sibling() {
            if next_node
                .node_name()
                .map_or(false, |name| name.as_ref() == "p")
            {
                br_node.remove_from_parent();
            }
        }
    }

    // Remove single-cell tables
    for table_node in article_sel.select("table").nodes().iter() {
        let tbody = if has_single_tag_inside_element(table_node, "tbody") {
            table_node.first_element_child().unwrap()
        } else {
            table_node.clone()
        };

        if has_single_tag_inside_element(&tbody, "tr") {
            let row = tbody.first_element_child().unwrap();
            if has_single_tag_inside_element(&row, "td") {
                let cell = row.first_element_child().unwrap();

                let new_tag = if cell.children().iter().all(|c| is_phrasing_content(c)) {
                    "p"
                } else {
                    "div"
                };

                cell.rename(new_tag);
                table_node.insert_before(&cell.id);
                table_node.remove_from_parent();
            }
        }
    }
}
