use std::collections::hash_map::HashMap;

use dom_query::{Document, NodeData, NodeRef};
use tendril::StrTendril;

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
        Self {
            doc: Document::from(html),
        }
    }
}

impl Readability {
    pub fn prepare(&mut self) {

        self.remove_empty_imgs();

        // remove scripts
        self.doc.select_matcher(&SCRIPT_MATCHER).remove();

        // remove styles
        self.doc.select_matcher(&STYLE_MATCHER).remove();

        // remove javascript urls
       self.doc.select_matcher(&UNWANTED_A_MATCHER).remove();

        // replace fonts with spans
        self.replace_fonts();

        // replace duplicating br elements
        self.replace_brs();
    }

    pub fn get_title(&self) -> StrTendril {
        self.doc.select_single_matcher(&TITLE_MATCHER).text()
    }

    fn replace_fonts(&mut self) {
        self.doc.select_matcher(&FONT_MATCHER).rename("span");
    }

    fn replace_brs(&mut self) {
        // TODO: revise this function
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
                    if let Some(node_name) = next.node_name() {
                        if node_name == "br".into() {
                            break;
                        }
                    }

                    if !is_phrasing_content(&next) {
                        break;
                    }

                    next_sibling = next.next_sibling();
                    next.remove_from_parent();
                    p.append_child(&next.id);
                }

                // TODO: is there any profit of this?
                while let Some(last) = p.last_child() {
                    if is_whitespace(&last) {
                        last.remove_from_parent();
                    }else {
                        break;
                    }
                }

                if let Some(parent) = p.parent() {
                    if let Some(node_name) = parent.node_name() {
                        if node_name == "p".into() {
                            parent.rename("div");
                        }
                    }
                }
            }
        }
    }

    
    fn remove_empty_imgs(&mut self) {
        // TODO: handle noscript images
        for mut sel in self.doc.select_matcher(&IMG_MATCHER).iter() {
            let attrs = sel.attrs();
            if attrs.iter().map(|a| &a.name.local).any(|k| k =="src" || k =="data-src" || k =="data-srcset" ||k == "srcset") {
                continue;
            }
            sel.remove();
        }
    }
    

    pub fn parse(&mut self) -> Article {
        self.prepare();

        Article {
            title: self.get_title(),
            content: self.doc.html(),
            text_content: self.doc.text(),
        }
    }

    pub fn parse_json_ld(&self) -> HashMap<String, String>{
        let mut meta_data = HashMap::new();
        for sel in self.doc.select_matcher(&JSONLD_MATCHER).iter() {
            if !meta_data.is_empty() {
                break;
            }
            // TODO: strip CDATA
            let content = sel.text();

            let context_val = gjson::get(&content, "@context");
            //TODO: what?
            let is_string = matches!(context_val.kind(), gjson::Kind::String);
            if !is_string ||  !RX_SCHEMA_ORG.is_match(context_val.str()) {
                break;
            }

            let mut article_type  = String::new();
            
            let type_val = gjson::get(&content, "@type");

            if !type_val.exists() {
                let type_val = gjson::get(&content, "@graph.#.@type");
                if matches!(type_val.kind(), gjson::Kind::String) {
                    article_type = type_val.str().to_string();
                }
            }else {
                article_type = type_val.str().to_string();
            }

            if RX_JSONLD_ARTICLE_TYPES.is_match(&article_type) {
                break;
            }

            let name_val = gjson::get(&content, "name");
            let headline_val = gjson::get(&content, "headline");
            let name_is_string = matches!(name_val.kind(), gjson::Kind::String);
            let headline_is_string = matches!(headline_val.kind(), gjson::Kind::String);
            
            let name = if name_is_string {
                name_val.str().trim().to_string()
            } else {
                String::new()
            };

            let headline = if headline_is_string  {
                 headline_val.str().trim().to_string()
            }else {
                String::new()
            };

            if name_is_string && headline_is_string && name != headline {
                todo!();
            } else if name_is_string {
                meta_data.insert("title".to_string(), name);
            } else if headline_is_string {
                meta_data.insert("title".to_string(), headline);
            }

            //Author

            let author_val = gjson::get(&content, "author");

            let byline = match author_val.kind() {
                gjson::Kind::Object => {
                    Some(author_val.get("name").str().trim().to_string())
                },
                gjson::Kind::Array => {
                    let names: Vec<String> = author_val.get("#.name").array().iter().map(|v| v.str().trim().to_string()).collect();
                    Some(names.join(", "))
                    
                },
                _ => None,
            };

            if let Some(byline) =  byline {
                meta_data.insert("author".to_string(), byline);
            }


            


        }
        return meta_data;
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

    if (node_name == "a")
        || (node_name == "del")
        || (node_name == "ins")
        || node.children().into_iter().all(|n| is_phrasing_content(&n))
    {
        return true;
    }

    false
}

fn is_whitespace(node: &NodeRef<NodeData>) -> bool {
    if node.is_text() {
        return node.text().trim().is_empty();
    }
    if node.is_element() {
        return node.node_name().map_or(false, |name| name == "br".into());
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

        debug_assert_eq!(
            readability.doc.select("span").html(),
            "<span>Styled Text Here</span>".into()
        );
    }

    #[test]
    fn test_remove_unwanted_urls()  {
        let contents = r#"<!DOCTYPE>
        <html>
            <head><title>Test</title></head>
            <body>
                <a href="/home">Home</a>
                <a href="javascript:void(0)">Click me</a>
            </body>
        </html>"#;
        let mut readability = Readability::from(contents);
        readability.prepare();
        assert_eq!(readability.doc.select("a").length(), 1);
    }
}
