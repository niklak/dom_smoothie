use dom_query::{Node, Selection};

use crate::glob::*;
use crate::helpers::*;
use crate::score::*;

fn clean(n: &Node, tag: &str) {
    let is_embed = EMBED_ELEMENTS.contains(&tag);

    let sel = Selection::from(n.clone()).select(tag);

    for node in sel.nodes().iter() {
        // Allow youtube and vimeo videos through as people usually want to see those.
        let mut should_remove = true;
        if is_embed {
            for attr in node.attrs().iter() {
                if RX_VIDEO_ATTRS.is_match(&attr.value) {
                    should_remove = false;
                    break;
                }
            }

            if node.node_name().unwrap().as_ref() == "object"
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
    let node_name = n.node_name().unwrap();
    if node_name.as_ref() == "svg" {
        return;
    }

    n.remove_attrs(PRESENTATIONAL_ATTRIBUTES);

    if DEPRECATED_SIZE_ATTRIBUTE_ELEMS.contains(&node_name.as_ref()) {
        n.remove_attrs(&["width", "height"]);
    }

    for child_node in n.element_children().iter() {
        clean_styles(child_node);
    }
}

fn should_clean_conditionally(sel: &Selection, tag: &str) -> bool {
    let node = sel.nodes().first().unwrap();
        let mut is_list = matches!(tag, "ul" | "ol");
        if !is_list {
            let list_length = sel
                .select("ul, ol")
                .iter()
                .fold(0, |acc, s| acc + s.inner_html().chars().count());
            is_list = (list_length as f32 / sel.inner_html().chars().count() as f32) > 0.9;
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
        if sel.select("table[data-readability-table]").exists(){
            return false;
        }

        let weight = get_class_weight(node);

        let mut content_score = 0.0;

        if node.text().matches(",").count() < 10 {
            // If there are not very many commas, and the number of
            // non-paragraph elements is more than paragraphs or other
            // ominous signs, remove the element.
            let p: f32 = sel.select("p").length() as f32;
            let img = sel.select("img").length() as f32;
            let li = sel.select("li").length() as f32 - 100.0;
            let input = sel.select("input").length() as f32;
            let heading_density = get_text_density(node, "h1,h2,h3,h4,h5,h6");

            let mut embed_count  = 0;

            let embeds_sel = sel.select("object,embed,iframe");

            for embed in embeds_sel.iter() {
                for attr in embed.attrs().iter() {
                    if RX_VIDEO_ATTRS.is_match(&attr.value) {
                        return false;
                    }
                }
                let embed_node = embed.nodes().first().unwrap();
                if embed_node.node_name().unwrap().as_ref() == "object" {
                    if RX_VIDEO_ATTRS.is_match(&embed_node.inner_html()) {
                        return false;
                    }
                }
                embed_count += 1;
            }

            let inner_text = sel.text();
            if RX_AD_WORDS.is_match(&inner_text) || RX_LOADING_WORDS.is_match(&inner_text) {
                return  true;
            }

            let content_length = inner_text.chars().count();
            let link_density = link_density(node);

            let mut textish_tags = vec!["span", "li", "td"];
            textish_tags.extend(BLOCK_ELEMS);

            let text_density = get_text_density(node, &textish_tags.join(","));
            let is_figure_child = has_ancestor_tag::<NodePredicate>(node, "figure", None, None);
        }
        false

}

fn clean_conditionally(node: &Node, tag: &str, clean: bool) {
    if !clean {
        return;
    }

    for sel in Selection::from(node.clone()).select(tag).iter() {
            if should_clean_conditionally(&sel, tag) {
                sel.remove();
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
        rows += rowspan.parse::<usize>().unwrap();

        //Now look for column-related info
        let mut columns_in_this_row = 0;

        for cell in tr.select("td").iter() {
            let colspan = cell.attr_or("colspan", "1");
            columns_in_this_row += colspan.parse::<usize>().unwrap();
        }
        cols = std::cmp::max(cols, columns_in_this_row);
    }

    (rows, cols)
}

fn mark_data_tables(n: &Node) {
    let table_sel = Selection::from(n.clone()).select("table");

    for sel in table_sel.iter() {
        let node = sel.nodes().first().unwrap();

        let role = node.attr_or("role", "");
        if role.as_ref() == "presentation" {
            set_data_readability_table(node, false);
            continue;
        }

        let data_table = node.attr_or("data-table", "");
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
        if rows == 1 && cols == 1 {
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
    for img_sel in Selection::from(n.clone())
        .select("img,picture,figure")
        .iter()
    {
        let node = img_sel.nodes().first().unwrap();
        let src = node.attr_or("src", "");
        // In some sites (e.g. Kotaku), they put 1px square image as base64 data uri in the src attribute.
        // So, here we check if the data uri is too short, just might as well remove it.
        if !src.is_empty() {
            if let Some(parts) = RX_BASE64_URL.captures(&src) {
                if parts.get(1).unwrap().as_str() == "image/svg+xml" {
                    continue;
                }

                // Make sure this element has other attributes which contains image.
                // If it doesn't, then this src is important and shouldn't be removed.
                let mut src_could_be_removed = false;

                for attr in node.attrs().iter() {
                    if attr.name.local.as_ref() == "src" {
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
                    let base64_starts = parts.get(0).unwrap().as_str().len();
                    let base64_len = src.len() - base64_starts;
                    if base64_len < 133 {
                        node.remove_attr("src");
                    }
                }
            }
        }

        // also check for "null" to work around https://github.com/jsdom/jsdom/issues/2580
        if (!src.is_empty() || node.has_attr("srcset")) && !node.has_class("lazy") {
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

                let tag_name = node.node_name().unwrap();
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

pub(crate) fn prep_article(article_content: &Node) {
    clean_styles(article_content);

    // Check for data tables before we continue, to avoid removing items in
    // those tables, which will often be isolated even though they're
    // visually linked to other content-ful elements (text, images, etc.).

    mark_data_tables(article_content);
    fix_lazy_images(article_content);

    // Clean out junk from the article content
    clean(article_content, "object");
    clean(article_content, "embed");
    clean(article_content, "footer");
    clean(article_content, "link");
    clean(article_content, "aside");
}
