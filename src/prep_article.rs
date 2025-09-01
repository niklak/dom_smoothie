use dom_query::{Node, Selection};
use flagset::FlagSet;

use crate::glob::*;
use crate::grab_flags::GrabFlags;
use crate::helpers::*;
use crate::matching::*;
use crate::score::*;
use crate::Config;

fn clean(root_sel: &Selection) {
    for node in root_sel.select_matcher(&MATCHER_CLEAN).nodes().iter() {
        // Allow youtube and vimeo videos through as people usually want to see those.
        let mut should_remove = true;

        if node_name_in(node, &EMBED_ELEMENTS) {
            for attr in node.attrs().iter() {
                if is_video_url(&attr.value) {
                    should_remove = false;
                    break;
                }
            }
            // For embed with <object> tag, check inner HTML as well.
            if should_remove && node.has_name("object") && is_video_url(&node.inner_html()) {
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

    if n.has_name("svg") {
        return;
    }

    n.remove_attrs(PRESENTATIONAL_ATTRIBUTES);

    if node_name_in(n, &DEPRECATED_SIZE_ATTRIBUTE_ELEMS) {
        n.remove_attrs(&["width", "height"]);
    }

    for child_node in n.element_children().iter() {
        clean_styles(child_node);
    }
}

fn should_clean_conditionally(node: &Node, flags: &FlagSet<GrabFlags>) -> bool {
    let sel = Selection::from(*node);
    // keep element if it has a data tables
    if sel.select_single_matcher(&MATCHER_DATA_TABLE).exists() {
        return false;
    }

    let is_data_table = |n: &Node| n.has_name("table") && n.has_attr("data-readability-table");

    if is_data_table(node) {
        return false;
    }
    // Next check if we're inside a data table, in which case don't remove it as well.
    if has_ancestor(node, None, is_data_table) {
        return false;
    }

    if has_ancestor(node, Some(0), |n| n.has_name("code")) {
        return false;
    }

    let weight = get_class_weight(node, flags.contains(GrabFlags::WeightClasses));

    if weight < 0.0 {
        return true;
    }

    let node_text = node.text();

    if node_text.matches(',').count() < 10 {
        // If there are not very many commas, and the number of
        // non-paragraph elements is more than paragraphs or other
        // ominous signs, remove the element.

        let mut embed_count = 0;

        let embeds_sel = sel.select_matcher(&MATCHER_EMBEDS);

        for embed in embeds_sel.nodes().iter() {
            for attr in embed.attrs().iter() {
                if is_video_url(&attr.value) {
                    return false;
                }
            }
            if embed.has_name("object") && is_video_url(&embed.inner_html()) {
                return false;
            }
            embed_count += 1;
        }
        let text_low = node_text.trim().to_lowercase();
        if AD_WORDS.contains(&text_low) || is_loading_word(&text_low) {
            return true;
        }

        let char_count = node.normalized_char_count();
        let Some(qual_name) = node.qual_name_ref() else {
            return false;
        };
        let tag = qual_name.local.as_ref();
        let mut is_list = matches!(tag, "ul" | "ol");
        if !is_list {
            let list_density = text_density(node, "ul,ol", Some(char_count));
            is_list = list_density > 0.9;
        }

        let img = node.find_descendants("img").len() as f32;

        let should_remove = || {
            let is_figure_child = has_ancestor(node, None, |n| n.has_name("figure"));
            let p = node.find_descendants("p").len() as f32;

            if !is_figure_child && img > 1.0 && p / img < 0.5 {
                return true;
            }

            // TODO: this is not working, useless
            let li = node.find_descendants("li").len() as f32 - 100.0;
            if !is_list && li > p {
                return true;
            }

            let input = node.find_descendants("input").len() as f32;
            if input > (p / 3.0).floor() {
                return true;
            }

            let link_density = link_density(node, Some(char_count));

            if !is_list
                && !is_figure_child
                && char_count < 25
                && (img == 0.0 || img > 2.0)
                && link_density > 0.0
                && text_density(node, "h1,h2,h3,h4,h5,h6", Some(char_count)) < 0.9
            {
                return true;
            }
            if !is_list && weight < 25.0 && link_density > 0.2 {
                return true;
            }

            if weight >= 25.0 && link_density > 0.5 {
                return true;
            }

            if (embed_count == 1 && char_count < 75) || embed_count > 1 {
                return true;
            }

            if img == 0.0 && text_density(node, TEXTISH_TAGS, Some(char_count)) == 0.0 {
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
            let li_count = node.find_descendants("li").len();
            return !(img == li_count as f32);
        }

        return have_to_remove;
    }
    false
}

fn clean_conditionally(sel: &Selection, tags: &str, flags: &FlagSet<GrabFlags>) {
    if !flags.contains(GrabFlags::CleanConditionally) {
        return;
    }

    let tag_sel = sel.select(tags);
    // traversing tag nodes in reverse order,
    // so that how children nodes will appear before parent nodes
    for tag_node in tag_sel.nodes().iter().rev() {
        if should_clean_conditionally(tag_node, flags) {
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
        // No need to adjust row count by the `row span` at all
        rows += 1;

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

fn mark_data_tables(base_sel: &Selection) {
    for table_node in base_sel.select_matcher(&MATCHER_TABLE).nodes() {
        if MINI_PRESENTATION.match_node(table_node) {
            set_data_readability_table(table_node, false);
            continue;
        }

        if MINI_AINT_DATA_TABLE.match_node(table_node) {
            set_data_readability_table(table_node, false);
            continue;
        }

        if table_node.has_attr("summary") {
            set_data_readability_table(table_node, true);
            continue;
        }

        let sel = Selection::from(*table_node);

        if sel.select_single_matcher(&MATCHER_TABLE_MEMBERS).exists() {
            set_data_readability_table(table_node, true);
            continue;
        }

        // nested tables indicate a layout table
        if sel.select_single_matcher(&MATCHER_TABLE).exists() {
            set_data_readability_table(table_node, false);
            continue;
        }

        let (rows, cols) = get_row_and_col_count(&sel);
        if rows == 1 || cols == 1 {
            set_data_readability_table(table_node, false);
            continue;
        }

        if rows >= 10 || cols > 4 {
            set_data_readability_table(table_node, true);
            continue;
        }
        set_data_readability_table(table_node, (rows * cols) > 10);
    }
}

fn fix_lazy_images(sel: &Selection) {
    for node in sel.select("img,picture,figure").nodes().iter() {
        // In some sites (e.g. Kotaku), they put 1px square image as base64 data uri in the src attribute.
        // So, here we check if the data uri is too short, just might as well remove it.
        if let Some(src) = node.attr("src") {
            if let Some((image_type, base64_data)) = split_base64_url(&src) {
                if image_type == "image/svg+xml" {
                    continue;
                }

                // Make sure this element has other attributes which contains image.
                // If it doesn't, then this src is important and shouldn't be removed.
                let mut src_could_be_removed = false;

                for attr in node.attrs().iter() {
                    if &attr.name.local == "src" {
                        continue;
                    }

                    if IMG_EXT.iter().any(|p| attr.value.contains(p)) {
                        src_could_be_removed = true;
                        break;
                    }
                }
                // Here we assume if image is less than 100 bytes (or 133 after encoded to base64)
                // it will be too small, therefore it might be placeholder image.
                if src_could_be_removed && base64_data.len() < 133 {
                    node.remove_attr("src");
                }
            }
        }

        if (node.has_attr("src") || node.has_attr("srcset")) && !MINI_LAZY.match_node(node) {
            continue;
        }

        for attr in node.attrs().iter() {
            if matches!(attr.name.local.as_ref(), "src" | "srcset" | "alt") {
                continue;
            }

            let mut copy_to: Option<&str> = None;
            let val = attr.value.to_ascii_lowercase();
            if is_img_attr_to_srcset(&val) {
                copy_to = Some("srcset");
            } else if is_img_attr_to_src(&val) {
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
                    let figure_sel = Selection::from(*node);
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

fn clean_headers(sel: &Selection, flags: &FlagSet<GrabFlags>) {
    for h_node in sel.select_matcher(&MATCHER_HEADING).nodes().iter() {
        if get_class_weight(h_node, flags.contains(GrabFlags::WeightClasses)) < 0.0 {
            h_node.remove_from_parent();
        }
    }
}

pub(crate) fn prep_article(article_node: &Node, flags: &FlagSet<GrabFlags>, cfg: &Config) {
    let article_sel = Selection::from(*article_node);
    // *Important*: Currently the order of calling 'cleaning' functions is matters.
    // It shouldn't be but it is.

    // Clean out elements with little content that have "share" in their id/class combinations from final top candidates,
    // which means we don't remove the top candidates even they have "share".
    remove_share_elements(&article_sel, cfg.char_threshold);

    // Check for data tables before we continue, to avoid removing items in
    // those tables, which will often be isolated even though they're
    // visually linked to other content-ful elements (text, images, etc.).
    mark_data_tables(&article_sel);

    fix_lazy_images(&article_sel);

    clean_conditionally(&article_sel, "form,fieldset", flags);

    // Clean out junk from the article content
    clean(&article_sel);

    clean_headers(&article_sel, flags);

    // Do these last as the previous stuff may have removed junk
    // that will affect these
    clean_conditionally(&article_sel, "table,ul,div", flags);

    // replace H1 with H2 as H1 should be only title that is displayed separately
    article_sel.select("h1").rename("h2");
    // remove all presentational attributes
    clean_styles(article_node);

    // Remove extra paragraphs

    // At this point, nasty iframes have been removed; only embedded video
    // ones remain.
    for p_node in article_sel.select("p").nodes() {
        let p_sel = Selection::from(*p_node);
        let content_el_count = p_sel.select("img,object,embed,iframe").length();
        if content_el_count == 0 && p_node.normalized_char_count() == 0 {
            p_sel.remove();
        }
    }

    for br_node in article_node.find_descendants("br") {
        if let Some(next_node) = br_node.next_element_sibling() {
            if next_node.has_name("p") {
                br_node.remove_from_parent();
            }
        }
    }

    fix_single_cell_tables(&article_sel);
}

fn fix_single_cell_tables(sel: &Selection) {
    // Remove single-cell tables
    for table_node in sel.select("table").nodes() {
        let tbody = if has_single_tag_inside_element(table_node, "tbody") {
            table_node.first_element_child().unwrap()
        } else {
            *table_node
        };

        if has_single_tag_inside_element(&tbody, "tr") {
            let row = tbody.first_element_child().unwrap();
            if has_single_tag_inside_element(&row, "td") {
                let cell = row.first_element_child().unwrap();

                let new_name = if cell.children().iter().all(|c| is_phrasing_content(c)) {
                    "p"
                } else {
                    "div"
                };
                cell.rename(new_name);
                table_node.replace_with(&cell);
            }
        }
    }
}

fn remove_share_elements(root_sel: &Selection, share_element_threshold: usize) {
    for child in root_sel.select("*[class],*[id]").nodes() {
        let mut has_share_elements = false;

        if child.normalized_char_count() >= share_element_threshold {
            continue;
        }

        if let Some(el) = child.element_ref() {
            has_share_elements = el.class().map_or(false, |s| contains_share_elements(&s));
            if !has_share_elements {
                has_share_elements = el.id().map_or(false, |s| contains_share_elements(&s));
            }
        }
        if has_share_elements {
            child.remove_from_parent();
        }
    }
}
