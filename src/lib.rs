use tendril::StrTendril;
use dom_query::{Document, NodeData, NodeRef};

mod glob;

use crate::glob::*;


pub struct Article {
    pub title: StrTendril,
    pub content: StrTendril,
    pub text_content: StrTendril,
}

pub struct Readability {
    pub doc: Document,
}

impl<T: Into<StrTendril>> From<T> for Readability {
    fn from(html: T) -> Self {
        Self {doc: Document::from(html)}
    }
}

impl Readability {
    pub fn prepare(&mut self)  {
        // remove scripts
        self.doc.select_matcher(&SCRIPT_MATCHER).remove();
        // remove styles
        self.doc.select_matcher(&STYLE_MATCHER).remove();

        // replace fonts
        self.replace_fonts();

        self.replace_brs();
    }

    pub fn get_title(&self) -> StrTendril {
        self.doc.select_single_matcher(&TITLE_MATCHER).text()
    }

    fn replace_fonts(&mut self)   {
        self.doc.select_matcher(&FONT_MATCHER).rename("span");
    }

    fn replace_brs(&mut self) {
        let sel = self.doc.select_matcher(&BR_MATCHER);

        for next_br in sel.nodes().iter() {
            let mut replaced = false;
            let mut next_sibling = next_br.next_sibling();
            while let Some(next) = next_sibling {
                let node_name = next.node_name();
                if node_name.is_none() {
                    break;
                }
                if next.node_name().unwrap() != "br".into() {
                    break;
                }
                replaced = true;
                next_sibling = next.next_sibling();
                next.remove_from_parent();
            }
            if replaced {
                let p = next_br;
                p.rename("p");

                let mut next_sibling = p.next_sibling();
                while let Some(next) = next_sibling {
                    if let Some(node_name) = next.node_name(){
                        if node_name == "br".into() {
                            break
                        }
                    }

                    if !is_phrasing_content(&next) {
                        break;
                    }

                    next_sibling = next.next_sibling();
                    next.remove_from_parent();
                    p.append_child(&next.id);
                }
                
            }

        }

        
    }

    

    pub fn parse(&mut self) -> Article {
        self.prepare();

        Article {
            title: self.get_title(),
            content: self.doc.html(),
            text_content: self.doc.text()
        }
    }
}

fn is_phrasing_content(node: &NodeRef<NodeData>) -> bool {
    // TODO: revise this function
    if node.is_text() {
        return true;
    }

    if !node.is_element() {
        return false;
    }

    let node_name_t = node.node_name().unwrap();
    let node_name: &str = &node_name_t;
    if PHRASING_ELEMS.contains(&node_name) {
        return true;
    }

    if (node_name == "a") || (node_name == "del") || (node_name == "ins") ||
        node.children().into_iter().all(|n| is_phrasing_content(&n)) {
            return true
    }


    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_font_tags() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                <font size="4" color="blue" face="Arial">Styled Text Here</font>
                <font size="4" color="blue" face="Arial">Styled Text Here</font>
            </body>
        </html>"#;

        let mut readability = Readability::from(contents);
        readability.replace_fonts();

        debug_assert_eq!(readability.doc.select("span").html(), "<span>Styled Text Here</span>".into());
        
    }

}