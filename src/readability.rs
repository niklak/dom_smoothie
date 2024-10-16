use std::collections::{HashMap, HashSet};

use dom_query::{Document, NodeData, NodeRef};
use tendril::StrTendril;

use crate::glob::*;

pub struct Article {
    pub title: StrTendril,
    pub content: StrTendril,
    pub text_content: StrTendril,
}

#[derive(Debug, Default, Clone, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
//TODO: better to convert each field to Option<String>
pub struct MetaData {
    pub title: String,
    pub byline: String,
    pub excerpt: String,
    pub site_name: String,
    pub published_time: Option<String>,
    pub modified_time: Option<String>,
    pub image: Option<String>,
    pub favicon: Option<String>,
    pub lang: Option<String>,
}

impl MetaData {
    fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.byline.is_empty()
            && self.excerpt.is_empty()
            && self.site_name.is_empty()
            && self.published_time.is_none()
            && self.modified_time.is_none()
            && self.image.is_none()
            && self.favicon.is_none()
            && self.lang.is_none()
    }

    fn unescape_html_entities(&mut self) {
        self.title = html_escape::decode_html_entities(&self.title).to_string();
        self.byline = html_escape::decode_html_entities(&self.byline).to_string();
        self.excerpt = html_escape::decode_html_entities(&self.excerpt).to_string();
        self.site_name = html_escape::decode_html_entities(&self.site_name).to_string();
        self.published_time = self
            .published_time
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
        self.modified_time = self
            .modified_time
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
        self.image = self
            .image
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
        self.favicon = self
            .favicon
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
    }
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

    pub fn get_article_title(&self) -> StrTendril {
        let orig_title = self
            .doc
            .select_single_matcher(&TITLE_MATCHER)
            .text()
            .trim()
            .to_string();
        let mut cur_title = orig_title.to_string();
        let char_count = orig_title.chars().count();
        let mut has_hierarchy_sep = false;

        if RX_TITLE_SEP.is_match(&orig_title) {
            has_hierarchy_sep = RX_HIERARCHY_SEP.is_match(&orig_title);

            let mut parts = RX_TITLE_SEP.splitn(&orig_title, 2);

            if let Some(first) = parts.next() {
                if first.split_whitespace().count() < 3 {
                    if let Some(last) = parts.next() {
                        cur_title = last.trim().to_string();
                    }
                } else {
                    cur_title = first.trim().to_string();
                }
            }
            // Everything below is such a mess
        } else if cur_title.contains(": ") {
            let matched = self.doc.select_matcher(&HEADINGS_MATCHER).iter().any(|h| {
                let text = h.text();
                text.trim() == cur_title
            });

            if !matched {
                if let Some(tmp_title) = orig_title
                    .rfind(":")
                    .map(|idx| orig_title[idx + 1..].trim().to_string())
                {
                    cur_title = tmp_title;
                    if cur_title.split_whitespace().count() < 3 {
                        if let Some(tmp_title) = orig_title
                            .find(":")
                            .map(|idx| orig_title[idx + 1..].trim().to_string())
                        {
                            cur_title = tmp_title
                        }
                    } else if orig_title
                        .find(":")
                        .map_or(0, |idx| orig_title[idx + 1..].split_whitespace().count())
                        > 5
                    {
                        cur_title = orig_title.to_string();
                    }
                }
            }
        } else if !(15..=150).contains(&char_count) {
            let h1_sel = self.doc.select_single("h1");
            if !h1_sel.is_empty() {
                cur_title = self.doc.select_single("h1").text().to_string();
            }
        }
        cur_title = normalize_spaces(&cur_title);

        // If we now have 4 words or fewer as our title, and either no
        // 'hierarchical' separators (\, /, > or ») were found in the original
        // title or we decreased the number of words by more than 1 word, use
        // the original title.
        let cur_title_wc = cur_title.split_whitespace().count();
        let orig_wc = RX_TITLE_ANY_SEP
            .replace_all(&orig_title, "")
            .split_whitespace()
            .count();

        if cur_title_wc <= 4 || (!has_hierarchy_sep || cur_title_wc != orig_wc - 1) {
            cur_title = orig_title;
        }

        cur_title.into()
    }

    fn replace_fonts(&mut self) {
        let mut sel = self.doc.select_matcher(&FONT_MATCHER);
        sel.rename("span");
        sel.remove_all_attrs();
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
                    } else {
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
            if attrs
                .iter()
                .map(|a| &a.name.local)
                .any(|k| k == "src" || k == "data-src" || k == "data-srcset" || k == "srcset")
            {
                continue;
            }
            sel.remove();
        }
    }

    pub fn parse(&mut self) -> Article {
        let ld_meta = self.parse_json_ld();

        self.prepare();

        let metadata = self.get_article_metadata(ld_meta);

        Article {
            title: metadata.title.into(),
            content: self.doc.html(),
            text_content: self.doc.text(),
        }
    }

    pub fn parse_json_ld(&self) -> Option<MetaData> {
        for sel in self.doc.select_matcher(&JSONLD_MATCHER).iter() {
            let text = sel.text();
            let content = RX_CDATA.replace_all(&text, "");

            /*
               Because of `gjson` reserved "@" symbol for its own modifiers,
               it is necessary to replace it with other symbol to be able of using `gjson`.
               Or decline using `gjson` at all and replace it with other crate.
               TODO: don't leave it like this!.
            */

            let content = content.trim().replace(r#""@"#, r#""^"#);

            let context_val = gjson::get(&content, "^context");
            // validating @context
            if !matches!(context_val.kind(), gjson::Kind::String)
                || !RX_SCHEMA_ORG.is_match(context_val.str())
            {
                break;
            }
            // validating @type
            let mut article_type = String::new();

            let type_val = gjson::get(&content, "^type");

            if !type_val.exists() {
                let type_val = gjson::get(&content, "^graph.#.^type");
                if matches!(type_val.kind(), gjson::Kind::String) {
                    article_type = type_val.str().to_string();
                }
            } else {
                article_type = type_val.str().to_string();
            }
            if !RX_JSONLD_ARTICLE_TYPES.is_match(&article_type) {
                break;
            }

            // Title
            let name_val = gjson::get(&content, "name");
            let headline_val = gjson::get(&content, "headline");
            let name_is_string = matches!(name_val.kind(), gjson::Kind::String);
            let headline_is_string = matches!(headline_val.kind(), gjson::Kind::String);

            let name = if name_is_string {
                name_val.str().trim().to_string()
            } else {
                String::new()
            };

            let headline = if headline_is_string {
                headline_val.str().trim().to_string()
            } else {
                String::new()
            };

            let mut ld_meta = MetaData::default();

            if name_is_string && headline_is_string && name != headline {
                let title = self.get_article_title();
                let name_matches = text_similarity(&name, &title) > 0.75;
                let headline_matches = text_similarity(&headline, &title) > 0.75;
                if headline_matches && !name_matches {
                    ld_meta.title = headline;
                } else {
                    ld_meta.title = name;
                }
            } else if name_is_string {
                ld_meta.title = name;
            } else if headline_is_string {
                ld_meta.title = headline;
            }

            //Author

            let author_val = gjson::get(&content, "author");

            let byline = match author_val.kind() {
                gjson::Kind::Object => Some(author_val.get("name").str().trim().to_string()),
                gjson::Kind::Array => {
                    let names: Vec<String> = author_val
                        .get("#.name")
                        .array()
                        .iter()
                        .map(|v| v.str().trim().to_string())
                        .collect();
                    Some(names.join(", "))
                }
                _ => None,
            };

            if let Some(byline) = byline {
                ld_meta.byline = byline;
            }

            // Description
            let excerpt_val = gjson::get(&content, "description");
            if matches!(excerpt_val.kind(), gjson::Kind::String) {
                ld_meta.excerpt = excerpt_val.str().trim().to_string();
            }

            // Publisher
            let publisher_val = gjson::get(&content, "publisher.name");
            if matches!(publisher_val.kind(), gjson::Kind::String) {
                ld_meta.site_name = publisher_val.str().trim().to_string();
            }

            // DatePublished
            let publisher_date_val = gjson::get(&content, "datePublished");
            if matches!(publisher_date_val.kind(), gjson::Kind::String) {
                ld_meta.published_time = Some(publisher_date_val.str().trim().to_string());
            }
            if !ld_meta.is_empty() {
                return Some(ld_meta);
            }
        }
        None
    }

    pub fn get_article_metadata(&self, json_ld: Option<MetaData>) -> MetaData {
        let mut values: HashMap<String, StrTendril> = HashMap::new();
        let mut metadata = json_ld.unwrap_or_default();

        let selection = self.doc.select_matcher(&META_MATCHER);

        for sel in selection.iter() {
            if let Some(content) = sel.attr("content") {
                // TODO: to trim or not to trim?
                let content: StrTendril = content.trim().into();
                let element_property = sel.attr("property");
                //TODO: looks like redundant checks!
                if let Some(property) = element_property {
                    let property: StrTendril = property.trim().into();
                    if RX_META_PROPERTY.is_match(&property) {
                        values.insert(property.to_string(), content.clone());
                    }
                }
                let element_name = sel.attr("name");
                if let Some(name) = element_name {
                    if RX_META_NAME.is_match(&name) {
                        values.insert(name.to_string(), content);
                    }
                }
            }
        }

        // title

        if metadata.title.is_empty() {
            if let Some(val) = get_map_any_value(&values, META_TITLE_KEYS) {
                metadata.title = val.to_string();
            }
        }

        //TODO: why? Leave till tests
        if metadata.title.is_empty() {
            metadata.title = self.get_article_title().to_string();
        }

        // author
        if metadata.byline.is_empty() {
            if let Some(val) = get_map_any_value(&values, META_BYLINE_KEYS) {
                metadata.byline = val.to_string();
            }
        }

        // description
        if metadata.excerpt.is_empty() {
            if let Some(val) = get_map_any_value(&values, META_EXCERPT_KEYS) {
                metadata.excerpt = val.to_string();
            }
        }

        //site name
        if metadata.site_name.is_empty() {
            if let Some(val) = values.get("og:site_name") {
                metadata.site_name = val.to_string();
            }
        }

        //published time
        if metadata.published_time.is_none() {
            metadata.published_time =
                get_map_any_value(&values, META_PUB_TIME_KEYS).map(|x| x.to_string());
        }

        self.assign_extra_article_metadata(&mut metadata, &values);

        metadata.lang = self.get_html_lang().map(|s| s.to_string());

        metadata.unescape_html_entities();
        metadata
    }

    fn assign_extra_article_metadata(
        &self,
        metadata: &mut MetaData,
        values: &HashMap<String, StrTendril>,
    ) {
        // thumbnail
        metadata.image = get_map_any_value(values, META_IMAGE_KEYS).map(|x| x.to_string());

        // modified time
        metadata.modified_time =
            get_map_any_value(values, META_MOD_TIME_KEYS).map(|x| x.to_string());

        //TODO: favicon
    }

    fn get_html_lang(&self) -> Option<StrTendril> {
        let sel = self.doc.select_single_matcher(&HTML_LANG_MATCHER);
        match sel.is_empty() {
            false => sel.attr("lang"),
            true => None,
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

fn text_similarity(text_a: &str, text_b: &str) -> f64 {
    //TODO: revise this later (use Jaccard index)
    let a = text_a.to_lowercase();
    let b = text_b.to_lowercase();
    let unique_tokens_a: HashSet<&str> = RX_TOKENIZE.split(&a).filter(|s| !s.is_empty()).collect();

    let tokens_b: Vec<&str> = RX_TOKENIZE.split(&b).filter(|s| !s.is_empty()).collect();

    let unique_tokens_b: Vec<&str> = tokens_b
        .iter()
        .filter(|&&s| !unique_tokens_a.contains(s))
        .cloned()
        .collect();

    let merged_b = tokens_b.join(" ");
    let merged_unique_b = unique_tokens_b.join(" ");

    let distance_b = merged_unique_b.chars().count() as f64 / merged_b.chars().count() as f64;
    1.0 - distance_b
}

fn normalize_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn get_map_any_value(map: &HashMap<String, StrTendril>, keys: &[&str]) -> Option<StrTendril> {
    keys.iter()
        .find_map(|&key| map.get(key))
        .map(|s| s.to_owned())
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
    fn test_remove_unwanted_urls() {
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

    #[test]
    fn test_text_similarity() {
        let text_a = "The quick brown fox";
        let text_b = "The quick fox";
        let similarity = text_similarity(text_a, text_b);
        assert!(similarity > 0.75);
    }

    #[test]
    fn test_text_similarity_similar() {
        let text_a = "THE QUICK FOX";
        let text_b = "The quick fox";
        let similarity = text_similarity(text_a, text_b);
        assert!(similarity == 1.0);
    }

    #[test]
    fn test_get_title() {
        let contents = include_str!("../test-pages/rustwiki_2024.html");
        let mut readability = Readability::from(contents);
        readability.prepare();

        let title = readability.get_article_title();

        assert_eq!(title, "Rust (programming language) - Wikipedia".into())
    }

    #[test]
    fn test_normalize_spaces() {
        let text = "  The    quick\t        brown\r\n  fox ";
        let normalized = normalize_spaces(text);
        assert_eq!(normalized, "The quick brown fox");
    }

    #[test]
    fn test_parse_json_ld() {
        let contents = include_str!("../test-pages/aclu/source.html");
        let ra = Readability::from(contents);

        let meta_contents = include_str!("../test-pages/aclu/expected_ld_meta.json");
        let expected_meta: MetaData = serde_json::from_str(&meta_contents).unwrap();

        let meta = ra.parse_json_ld().unwrap();

        assert_eq!(expected_meta, meta);
    }
}
