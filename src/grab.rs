use dom_query::{Document, Node, NodeData, NodeRef};

use crate::glob::*;

//TODO: do not forget FLAGS

pub fn grab_article(doc: &Document) {
    clean_doc(doc);
    for node in doc.select("*").nodes() {
        let matching_string = get_node_matching_string(node);
        if !is_probably_visible(node) {
            node.remove_from_parent();
            continue;
        }
    }
}

fn clean_doc(doc: &Document) {
    //remove by selection in any case
    // User is not able to see elements applied with both "aria-modal = true" and "role = dialog"
    doc.select_matcher(&DIALOGS_MATCHER).remove();

    // Remove DIV, SECTION, and HEADER nodes without any content(e.g. text, image, video, or iframe).
    doc.select_matcher(&EMPTY_SECTION_MATCHER).remove();
}

fn get_node_matching_string(node: &NodeRef<NodeData>) -> String {
    let class = node.attr("class");
    let id = node.attr("id");
    class
        .zip(id)
        .map_or_else(String::new, |(a, b)| format!("{a} {b}"))
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

#[cfg(test)]
mod tests {
    use super::*;

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
        grab_article(&doc);

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
        assert_eq!(sel.nodes().iter().filter(|n| n.is_element()).count(), 10);
        clean_doc(&doc);
        assert_eq!(
            doc.select("body > *")
                .nodes()
                .iter()
                .filter(|n| n.is_element())
                .count(),
            1
        );
    }
}
