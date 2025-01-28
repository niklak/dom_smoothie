use foldhash::HashMap;

use dom_query::{Document, Node, NodeData, NodeRef, Selection};
use tendril::StrTendril;
use url::Url;

use crate::config::TextMode;
use crate::glob::*;
use crate::helpers::*;
use crate::is_probably_readable;
use crate::text_helpers::format_text;
use crate::Config;
use crate::ReadabilityError;

/// This struct represents the content of the article
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Article {
    /// The title
    pub title: String,
    /// The author
    pub byline: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::serde_helpers::serialize_str_tendril",
            deserialize_with = "crate::serde_helpers::deserialize_str_tendril"
        )
    )]
    /// The relevant HTML content
    pub content: StrTendril,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::serde_helpers::serialize_str_tendril",
            deserialize_with = "crate::serde_helpers::deserialize_str_tendril"
        )
    )]
    /// The relevant text content
    pub text_content: StrTendril,
    /// The text length
    pub length: usize,
    /// The excerpt
    pub excerpt: Option<String>,
    /// The name of the site
    pub site_name: Option<String>,
    /// The text direction
    pub dir: Option<String>,
    /// The document language
    pub lang: Option<String>,
    /// The published time of the document
    pub published_time: Option<String>,
    /// The modified time of the document
    pub modified_time: Option<String>,
    /// The image of the document
    pub image: Option<String>,
    /// The favicon of the document
    pub favicon: Option<String>,
    /// The metadata's url
    pub url: Option<String>,
}

/// This struct represents the metadata extracted from the document
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Metadata {
    pub title: String,
    pub byline: Option<String>,
    pub excerpt: Option<String>,
    pub site_name: Option<String>,
    pub published_time: Option<String>,
    pub modified_time: Option<String>,
    pub image: Option<String>,
    pub favicon: Option<String>,
    pub lang: Option<String>,
    pub url: Option<String>,
    pub dir: Option<String>,
}

impl Metadata {
    fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.byline.is_none()
            && self.excerpt.is_none()
            && self.site_name.is_none()
            && self.published_time.is_none()
            && self.modified_time.is_none()
            && self.image.is_none()
            && self.favicon.is_none()
            && self.lang.is_none()
    }

    fn unescape_html_entities(&mut self) {
        self.title = html_escape::decode_html_entities(&self.title).to_string();
        self.byline = self
            .byline
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
        self.excerpt = self
            .excerpt
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
        self.site_name = self
            .site_name
            .as_ref()
            .map(|s| html_escape::decode_html_entities(&s).to_string());
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

/// A struct that provides readability functionality
pub struct Readability {
    /// The [Document] to be processed
    pub doc: Document,
    /// The base URL of the document
    pub doc_url: Option<url::Url>,
    /// Configuration options for the readability
    pub config: Config,
}

impl<T: Into<StrTendril>> From<T> for Readability {
    fn from(html: T) -> Self {
        Self {
            doc: Document::from(html),
            doc_url: None,
            config: Config::default(),
        }
    }
}

impl Readability {
    /// Create a new `Readability` instance
    ///
    /// # Panics
    ///
    /// Panics if `document_url` is not a valid URL
    ///
    /// # Arguments
    ///
    /// - `html` -- HTML content
    /// - `document_url` -- a base URL of the page
    /// - `cfg` -- an optional `Config` instance
    ///
    /// # Returns
    ///
    /// A new [`Readability`] instance
    ///
    /// # Errors
    ///
    /// Returns [`ReadabilityError::BadDocumentURL`] if `document_url` is not a valid URL
    pub fn new<T: Into<StrTendril>>(
        html: T,
        document_url: Option<&str>,
        cfg: Option<Config>,
    ) -> Result<Self, ReadabilityError> {
        Self::with_document(Document::from(html), document_url, cfg)
    }

    /// Create a new `Readability` instance with a `dom_query::Document`
    ///
    /// # Arguments
    ///
    /// - `document` -- a `dom_query::Document` instance
    /// - `document_url` -- a base URL of the page
    /// - `cfg` -- an optional `Config` instance
    ///
    /// # Returns
    ///
    /// A new [`Readability`] instance
    ///
    /// # Errors
    ///
    /// Returns [`ReadabilityError::BadDocumentURL`] if `document_url` is not a valid URL
    pub fn with_document(
        document: dom_query::Document,
        document_url: Option<&str>,
        cfg: Option<Config>,
    ) -> Result<Self, ReadabilityError> {
        let doc_url = if let Some(u) = document_url {
            Some(Url::parse(u)?)
        } else {
            None
        };

        Ok(Self {
            doc: document,
            doc_url,
            config: cfg.unwrap_or_default(),
        })
    }
}

impl Readability {
    /// Prepares the document for parsing:
    ///
    /// 1. Remove empty images
    /// 2. Unwrap images inside `noscript` tags
    /// 3. Remove `script` tags
    /// 4. Remove `style` tags
    /// 5. Replace multiple `br` tags with a single `br` tag
    /// 6. Replace `font` tags with `span` tags
    /// 7. Remove comments
    fn prepare(&mut self) {
        self.remove_empty_imgs();

        self.unwrap_noscript_images();

        // remove scripts
        self.doc.select_matcher(&MATCHER_SCRIPT).remove();

        // remove styles
        self.doc.select_matcher(&MATCHER_STYLE).remove();

        // replace duplicating br elements
        self.replace_brs();

        // replace fonts with spans
        self.replace_fonts();

        // remove comments
        self.remove_comments();
    }

    /// Return the title of the article as a `StrTendril`.
    ///
    /// This method will try to guess the title of the article by looking at the
    /// content of the page. It will first look for a `<title>` tag in the HTML
    /// document, and if it doesn't find it, it will look for a `<h1>` tag.
    /// If it still doesn't find one, it will return an empty string.
    ///
    /// The method will also try to clean up the title by removing any
    /// unnecessary characters from it.
    pub fn get_article_title(&self) -> StrTendril {
        let orig_title = self
            .doc
            .select_single_matcher(&MATCHER_TITLE)
            .text()
            .trim()
            .to_string();
        //let orig_title = normalize_spaces(&orig_title);
        let mut cur_title = orig_title.to_string();
        let char_count = orig_title.chars().count();
        let mut has_hierarchy_sep = false;
        //TODO: handle `—` or not?
        if RX_TITLE_SEP.is_match(&orig_title) {
            has_hierarchy_sep = RX_HIERARCHY_SEP.is_match(&orig_title);
            cur_title = RX_TITLE_W_LAST.replace(&orig_title, "$1").to_string();

            if cur_title.split_whitespace().count() < 3 {
                cur_title = RX_TITLE_W_FIRST.replace(&orig_title, "$1").to_string();
            }
            // Everything below is such a mess
        } else if cur_title.contains(": ") {
            let matched = self.doc.select_matcher(&MATCHER_HEADING).iter().any(|h| {
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
                        .map_or(0, |idx| orig_title[0..idx + 1].split_whitespace().count())
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
        if cur_title_wc <= 4 && (!has_hierarchy_sep || cur_title_wc != orig_wc - 1) {
            cur_title = orig_title;
        }

        cur_title.into()
    }

    fn replace_fonts(&mut self) {
        let sel = self.doc.select_matcher(&MATCHER_FONT);
        sel.rename("span");
        sel.remove_all_attrs();
    }

    fn replace_brs(&mut self) {
        let sel = self.doc.select_matcher(&MATCHER_BR);

        for br in sel.nodes().iter() {
            let mut next_sibling = br.next_sibling();
            let mut replaced = false;

            while let Some(next) = next_significant_node(next_sibling) {
                let Some(node_name) = next.node_name() else {
                    break;
                };
                if node_name != "br".into() {
                    break;
                }

                replaced = true;
                next_sibling = next.next_sibling();
                next.remove_from_parent();
            }
            if replaced {
                let p = br.tree.new_element("p");
                br.replace_with(&p);

                let mut next_sibling = p.next_sibling();
                while let Some(next) = next_sibling {
                    if let Some(node_name) = next.node_name() {
                        if node_name == "br".into() {
                            let next_elem = next_significant_node(next.next_sibling());
                            if let Some(elem_name) = next_elem.and_then(|n| n.node_name()) {
                                if elem_name == "br".into() {
                                    break;
                                }
                            }
                        }
                    }

                    if !is_phrasing_content(&next) {
                        break;
                    }

                    next_sibling = next.next_sibling();
                    p.append_child(&next);
                }

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
        for sel in self.doc.select_matcher(&MATCHER_IMG).iter() {
            let attrs = sel.attrs();
            if attrs
                .iter()
                .map(|a| &a.name.local)
                .any(|k| matches!(k.as_ref(), "src" | "data-src" | "data-srcset" | "srcset"))
            {
                continue;
            }
            if attrs.iter().any(|a| RX_IMG_ATTR.is_match(&a.value)) {
                continue;
            }

            sel.remove();
        }
    }

    fn unwrap_noscript_images(&self) {
        let noscript_sel = self.doc.select("noscript:has(img:only-child)");
        for noscript_node in noscript_sel.nodes().iter() {
            let Some(prev_sibling) = noscript_node.prev_element_sibling() else {
                continue;
            };
            let prev_sel = Selection::from(prev_sibling.clone());
            let prev_img: NodeRef;
            if prev_sel.is("img") {
                prev_img = prev_sibling;
            } else if prev_sel.is("*:has( > img:only-child)") {
                let prev_sel_img = prev_sel.select("img:only-child");
                prev_img = prev_sel_img.nodes()[0].clone();
            } else {
                continue;
            }
            let noscript_img_sel = Selection::from(noscript_node.clone()).select("img");
            // at this point noscript_img_sel always has one element
            let new_img = &noscript_img_sel.nodes()[0];

            for attr in prev_img.attrs() {
                if attr.value.as_ref() == "" {
                    continue;
                }

                if matches!(attr.name.local.as_ref(), "src" | "srcset")
                    || RX_IMG_ATTR.is_match(&attr.value)
                {
                    if new_img.attr_or(&attr.name.local, "") == attr.value {
                        continue;
                    }
                    if new_img.has_attr(&attr.name.local) {
                        let attr_name = format!("data-old-{}", attr.name.local);
                        new_img.set_attr(&attr_name, &attr.value);
                    } else {
                        new_img.set_attr(&attr.name.local, &attr.value);
                    };
                }
            }
            prev_img.replace_with(new_img);
        }
    }

    /// Extracts the relevant content from the document and provides it as a [`Article`] object.
    ///
    /// This is the primary method of the crate. It performs the following steps:
    ///
    /// - Verify the document (element nodes length)
    /// - Extracts the metadata
    /// - Cleans the document
    /// - Extracts the main content of the document
    /// - Post-processes the content
    /// - Returns the content and the metadata as an [`Article`] object
    ///
    /// # Returns
    ///
    /// An [`Article`] object containing the content and the metadata.
    ///
    /// # Errors
    /// If `config.max_elements_to_parse` is > 0 and the document's number of element nodes exceeds this limit,
    /// a [`ReadabilityError::TooManyElements`] error is returned.
    /// If the document fails to extract the content, a [`ReadabilityError::GrabFailed`] error is returned.
    pub fn parse(&mut self) -> Result<Article, ReadabilityError> {
        self.verify_doc()?;

        let ld_meta = if self.config.disable_json_ld {
            None
        } else {
            self.parse_json_ld()
        };
        let mut metadata = self.get_article_metadata(ld_meta);

        self.prepare();

        let Some(doc) = self.grab_article(&mut metadata) else {
            return Err(ReadabilityError::GrabFailed);
        };

        let root_sel = doc.select_single_matcher(&MATCHER_CONTENT_ID);

        let Some(root_node) = root_sel.nodes().first() else {
            // After `grab_article` successfully returns a document
            // there is no way that `#readability-page-1 does not exists
            return Err(ReadabilityError::GrabFailed);
        };

        // Getting a base uri from the Readability.document,
        // which wasn't changed after the grabbing the article
        let base_url = self.parse_base_url();
        self.post_process_content(&root_sel, base_url);

        // If we haven't found an excerpt in the article's metadata, use the article's
        // first paragraph as the excerpt. This is used for displaying a preview of
        // the article's content.

        if metadata.excerpt.is_none() {
            // TODO: Although this matches readability.js,
            // the procedure is far from perfect and requires improvement.
            metadata.excerpt = extract_excerpt(&doc)
        }

        let text_content = match self.config.text_mode {
            TextMode::Raw => root_node.text(),
            TextMode::Formatted => format_text(root_node),
        };
        let text_length = text_content.chars().count();

        Ok(Article {
            title: metadata.title,
            byline: metadata.byline,
            dir: metadata.dir,
            lang: metadata.lang,
            content: root_sel.html(),
            text_content,
            length: text_length,
            excerpt: metadata.excerpt,
            site_name: metadata.site_name,
            published_time: metadata.published_time,
            modified_time: metadata.modified_time,
            image: metadata.image,
            favicon: metadata.favicon,
            url: metadata.url,
        })
    }

    /// This method will search for a JSON-LD block in the page and
    /// extract the metadata from it.
    ///
    /// # Returns
    ///
    /// A [Metadata] object containing the metadata extracted from the JSON-LD block.
    /// If no valid JSON-LD block is found, this method returns `None`.
    pub fn parse_json_ld(&self) -> Option<Metadata> {
        for sel in self.doc.select_matcher(&MATCHER_JSONLD).iter() {
            let text = sel.text();
            let content = RX_CDATA.replace_all(&text, "");

            /*
               Because of `gjson` reserved "@" symbol for its own modifiers,
               it is necessary to replace it with other symbol to be able of using `gjson`.
               Or decline using `gjson` at all and replace it with other crate.
               TODO: don't leave it like this!.
            */

            let content = content.trim().replace(r#""@"#, r#""^"#);

            let mut parsed = gjson::parse(&content);
            let clipped_content: String;

            if parsed.kind() == gjson::Kind::Array {
                for it in parsed.array().iter() {
                    let typ = it.get("^type");
                    if typ.kind() == gjson::Kind::String
                        && JSONLD_ARTICLE_TYPES.iter().any(|p| typ.str().contains(p))
                    {
                        clipped_content = it.to_string();
                        parsed = gjson::parse(&clipped_content);
                        break;
                    }
                }
            }

            let mut context_matched = false;

            let context_val = parsed.get("^context");
            if context_val.kind() == gjson::Kind::String
                && RX_SCHEMA_ORG.is_match(context_val.str())
            {
                // validating @context
                context_matched = true;
            }

            let context_vocab = parsed.get("^context.^vocab");
            if context_vocab.kind() == gjson::Kind::String
                && RX_SCHEMA_ORG.is_match(context_vocab.str())
            {
                // validating @context
                context_matched = true;
            }

            if !context_matched {
                continue;
            }

            // validating @type
            let mut article_type = String::new();

            let type_val = parsed.get("^type");
            //There are no examples with @graph array, so it is not clear how to check it
            //TODO: implement same @graph logic as mozilla, when there will be examples.
            if !type_val.exists() {
                let type_val = parsed.get("^graph.#.^type");
                if matches!(type_val.kind(), gjson::Kind::String) {
                    article_type = type_val.str().to_string();
                }
            } else {
                article_type = type_val.str().to_string();
            }
            if !JSONLD_ARTICLE_TYPES
                .iter()
                .any(|p| article_type.contains(p))
            {
                continue;
            }

            // Title
            let name_val = parsed.get("name");
            let headline_val = parsed.get("headline");
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

            let mut ld_meta = Metadata::default();

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

            let author_val = parsed.get("author");

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

            if let Some(byline) = byline.filter(|s| !s.is_empty()) {
                ld_meta.byline = Some(byline);
            }

            // Description
            ld_meta.excerpt = get_json_ld_string_value(&parsed, "description");

            // Publisher
            ld_meta.site_name = get_json_ld_string_value(&parsed, "publisher.name");

            // DatePublished
            ld_meta.published_time = get_json_ld_string_value(&parsed, "datePublished");

            // DateModified
            ld_meta.modified_time = get_json_ld_string_value(&parsed, "dateModified");

            // Url
            ld_meta.url = get_json_ld_string_value(&parsed, "url");

            // Image
            ld_meta.image = get_json_ld_string_value(&parsed, "image");

            if !ld_meta.is_empty() {
                return Some(ld_meta);
            }
        }
        None
    }

    /// Extracts metadata from a web page.
    ///
    /// This function takes into account standard metadata formats like OpenGraph, Dublin Core,
    /// schema.org, and also tries to extract some metadata from HTML tags like `<title>`.
    ///
    /// The function takes an optional `Metadata` object as input, which is used as a fallback
    /// if no metadata can be found on the page. If the input `Metadata` object contains any
    /// of the following fields, they will not be overwritten by this function:
    /// - `title`
    /// - `byline`
    /// - `excerpt`
    /// - `site_name`
    /// - `published_time`
    /// - `lang`
    ///
    /// # Arguments
    ///
    /// - `json_ld` -- An optional [`Metadata`] object, containing metadata extracted from JSON-LD.
    ///
    /// # Returns
    ///
    /// A [`Metadata`] object containing the extracted metadata.
    pub fn get_article_metadata(&self, json_ld: Option<Metadata>) -> Metadata {
        let mut values: HashMap<String, StrTendril> = HashMap::default();
        let mut metadata = json_ld.unwrap_or_default();

        let selection = self.doc.select_matcher(&MATCHER_META);

        for node in selection.nodes().iter() {
            if let Some(content) = node.attr("content") {
                let content = content.trim();
                if content.is_empty() {
                    continue;
                }
                if let Some(property) = node.attr("property") {
                    let property = property.trim();
                    if RX_META_PROPERTY.is_match(property) {
                        if let Some(caps) = RX_META_PROPERTY.captures(property) {
                            let k = caps[0].to_string().trim().to_string();
                            values.insert(k, content.into());
                        }
                    }
                }
                if let Some(name) = node.attr("name") {
                    if RX_META_NAME.is_match(&name) {
                        values.insert(normalize_meta_key(&name), content.into());
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

        if metadata.title.is_empty() {
            metadata.title = self.get_article_title().to_string();
        }

        // author
        if metadata.byline.is_none() {
            if let Some(val) = get_map_any_value(&values, META_BYLINE_KEYS) {
                metadata.byline = Some(val.to_string());
            }
            // if metadata is still none
            if metadata.byline.is_none() {
                if let Some(v) = values.get("article:author") {
                    if url::Url::parse(v).is_err() {
                        metadata.byline = Some(v.to_string());
                    }
                }
            }
        }

        // description
        if metadata.excerpt.is_none() {
            if let Some(val) = get_map_any_value(&values, META_EXCERPT_KEYS) {
                metadata.excerpt = Some(val.to_string());
            }
        }

        //site name
        if metadata.site_name.is_none() {
            if let Some(val) = values.get("og:site_name") {
                metadata.site_name = Some(val.to_string());
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
        metadata: &mut Metadata,
        values: &HashMap<String, StrTendril>,
    ) {
        // thumbnail
        if metadata.image.is_none() {
            metadata.image = get_map_any_value(values, META_IMAGE_KEYS).map(|x| x.to_string());
        }

        // modified time
        if metadata.modified_time.is_none() {
            metadata.modified_time =
                get_map_any_value(values, META_MOD_TIME_KEYS).map(|x| x.to_string());
        }
        //TODO: favicon
    }

    fn remove_comments(&self) {
        remove_comments(&self.doc.root());
    }

    fn get_html_lang(&self) -> Option<StrTendril> {
        let sel = self.doc.select_single_matcher(&MATCHER_HTML_LANG);
        match sel.is_empty() {
            false => sel.attr("lang"),
            true => None,
        }
    }

    fn post_process_content(&self, root_sel: &Selection, base_url: Option<url::Url>) {
        // Readability cannot open relative uris so we convert them to absolute uris.

        self.fix_js_links(root_sel);

        self.fix_relative_uris(root_sel, base_url);

        simplify_nested_elements(root_sel);

        let score_sel = root_sel
            .parent()
            .select("*[data-readability-score], *[data-readability-table]");
        score_sel.remove_attrs(&["data-readability-score", "data-readability-table"]);

        if !self.config.keep_classes {
            self.clean_classes(root_sel);
        }
    }

    fn clean_classes(&self, sel: &Selection) {
        if self.config.classes_to_preserve.is_empty() {
            sel.select("*[class]").remove_attr("class");
            return;
        }
        let classes_to_preserve: Vec<&str> = self
            .config
            .classes_to_preserve
            .iter()
            .map(|s| s.as_str())
            .collect();

        let class_selector = classes_to_preserve
            .iter()
            .map(|s| format!(".{}", s))
            .collect::<Vec<String>>()
            .join(",");

        let other_class_sel = sel.select(&format!(".page *[class]:not({})", class_selector));
        other_class_sel.remove_attr("class");

        let class_sel = sel.select(&format!(".page {}", class_selector));

        for node in class_sel.nodes().iter() {
            let Some(class_string) = node.attr("class") else {
                unreachable!();
            };
            let classes_to_remove = class_string
                .split_whitespace()
                .filter(|s| !classes_to_preserve.contains(s))
                .collect::<Vec<&str>>()
                .join(" ");
            node.remove_class(classes_to_remove.as_str());
        }
    }

    fn fix_js_links(&self, root_sel: &Selection) {
        // Handle links with javascript: URIs, since
        // they won't work after scripts have been removed from the page.
        for a in root_sel.select_matcher(&MATCHER_JS_LINK).nodes().iter() {
            let children = a.children();
            if children.len() == 1 {
                let child = &children[0];
                if child.is_text() {
                    a.replace_with(child);
                } else {
                    a.remove_all_attrs();
                    a.rename("span");
                }
            } else if children.is_empty() {
                a.remove_from_parent();
            } else {
                a.remove_all_attrs();
                a.rename("span");
            }
        }
    }

    fn fix_relative_uris(&self, root_sel: &Selection, base_url: Option<url::Url>) {
        let url_sel =
            if base_url.as_ref().map(|u| u.as_str()) == self.doc_url.as_ref().map(|u| u.as_str()) {
                r##"a[href]:not([href^="#"]):not([href^="http"])"##
            } else {
                r##"a[href]:not([href^="http"])"##
            };
        if let Some(base_url) = base_url {
            for a in root_sel.select(url_sel).nodes().iter() {
                let Some(href) = a.attr("href") else {
                    unreachable!();
                };
                let abs_url = to_absolute_url(&href, &base_url);
                a.set_attr("href", abs_url.as_str());
            }

            for media in root_sel.select_matcher(&MATCHER_SOURCES).nodes().iter() {
                if let Some(src) = media.attr("src") {
                    let abs_src = to_absolute_url(&src, &base_url);
                    media.set_attr("src", abs_src.as_str());
                }

                if let Some(poster) = media.attr("poster") {
                    let abs_poster = to_absolute_url(&poster, &base_url);
                    media.set_attr("poster", abs_poster.as_str());
                }

                if let Some(srcset) = media.attr("srcset") {
                    let abs_srcset: Vec<String> = srcset
                        .split(", ")
                        .map(|s| {
                            if let Some((src, cond)) = s.split_once(" ") {
                                let abs_src = to_absolute_url(src.trim(), &base_url);
                                format!("{} {}", abs_src, cond)
                            } else {
                                s.to_string()
                            }
                        })
                        .collect();
                    media.set_attr("srcset", abs_srcset.join(", ").as_str());
                }
            }
        }
    }

    fn parse_base_url(&self) -> Option<url::Url> {
        let Some(base_uri) = self.doc.base_uri() else {
            return self.doc_url.clone();
        };

        if let Some(doc_url) = self.doc_url.clone() {
            doc_url.join(&base_uri).ok()
        } else {
            url::Url::parse(&base_uri).ok()
        }
    }

    fn verify_doc(&self) -> Result<(), ReadabilityError> {
        if self.config.max_elements_to_parse > 0 {
            let total_elements = self
                .doc
                .root()
                .descendants_it()
                .filter(|n| n.is_element())
                .count();
            if total_elements > self.config.max_elements_to_parse {
                return Err(ReadabilityError::TooManyElements(
                    total_elements,
                    self.config.max_elements_to_parse,
                ));
            }
        }
        Ok(())
    }

    /// Estimates whether the document is readable in a *quick-and-dirty* way.
    ///
    /// Must be called before `Readability::parse` because it cleans the document and changes its structure.
    ///
    /// # Returns
    ///
    /// True if the document is readable, false otherwise.
    pub fn is_probably_readable(&self) -> bool {
        is_probably_readable(
            &self.doc,
            Some(self.config.readable_min_score),
            Some(self.config.readable_min_content_length),
        )
    }
}

fn get_map_any_value<'a>(map: &'a HashMap<String, StrTendril>, keys: &[&str]) -> Option<&'a StrTendril> {
    keys.iter()
        .find_map(|&key| map.get(key))
}

fn remove_comments(n: &Node) {
    let mut ops = n.children();
    let mut comments = vec![];
    while let Some(node) = ops.pop() {
        node.query(|n| match n.data {
            NodeData::Comment { .. } => {
                comments.push(node.clone());
            }
            NodeData::Element(_) => {
                ops.extend(node.children());
            }
            _ => {}
        });
    }

    for comment in comments {
        comment.remove_from_parent();
    }
}

fn next_significant_node(node: Option<NodeRef>) -> Option<NodeRef> {
    let mut next = node;
    while let Some(ref n) = next {
        if !n.is_element() && n.text().trim().is_empty() {
            next = n.next_sibling();
        } else {
            break;
        }
    }
    next
}

fn simplify_nested_elements(root_sel: &Selection) {
    for td_node in root_sel.select("*:not(tr) > td").nodes().iter() {
        if let Some(parent) = td_node.parent() {
            if let Some(first_child) = td_node.first_child() {
                parent.append_children(&first_child);
            }
        }
        td_node.remove_from_parent();
    }

    let only_sel = root_sel
        .select("div, section")
        .select(":is(div, section) > :is(div, section):only-child");

    for node in only_sel.nodes().iter().rev() {
        let Some(parent) = node.parent() else {
            unreachable!();
        };
        for attr in parent.attrs() {
            node.set_attr(&attr.name.local, &attr.value);
        }
        parent.replace_with(&node.id);
    }
    root_sel.select(":is(div, section):empty").remove();
}

fn extract_excerpt(doc: &Document) -> Option<String> {
    let p_sel = doc
        .select_single("#readability-page-1")
        .select_single_matcher(&MATCHER_P);
    if p_sel.is_empty() {
        None
    } else {
        Some(p_sel.text().trim().to_string())
    }
}

fn normalize_meta_key(raw_key: &str) -> String {
    raw_key
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .replace('.', ":")
}

fn get_json_ld_string_value(value: &gjson::Value, path: &str) -> Option<String> {
    let val = value.get(path);
    if matches!(val.kind(), gjson::Kind::String) {
        let val = val.str().trim().to_string();
        if !val.is_empty() {
            return Some(val);
        }
    }
    None
}

fn to_absolute_url(raw_url: &str, base_uri: &Url) -> String {
    let u = if raw_url.starts_with("file://") {
        raw_url.replace("|/", ":/")
    } else {
        raw_url.to_string()
    };

    base_uri.join(&u).map_or(u, |uri| uri.to_string())
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
        let readability = Readability::from(contents);
        readability.fix_js_links(&readability.doc.select("body"));
        assert_eq!(readability.doc.select("a").length(), 1);
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
    #[cfg(feature = "serde")]
    fn test_parse_json_ld() {
        let contents = include_str!("../test-pages/ok/aclu/source.html");
        let ra = Readability::from(contents);

        let meta_contents = include_str!("../test-pages/aclu_ld_meta.json");
        let expected_meta: Metadata = serde_json::from_str(meta_contents).unwrap();

        let meta = ra.parse_json_ld().unwrap();

        assert_eq!(expected_meta, meta);
    }

    #[test]
    fn test_disable_sparse_json_ld() {
        let contents = include_str!("../test-pages/rustwiki_2024.html");
        let cfg = Config {
            disable_json_ld: false,
            ..Default::default()
        };
        let mut readability = Readability::new(contents, None, Some(cfg)).unwrap();
        // `Article::url` is always taken from JSON-LD.
        // Therefore, if `config.disable_json_ld` is set to true, `Article::url` will be `None`.
        let res = readability.parse().unwrap();
        let expected_url =
            Some("https://en.wikipedia.org/wiki/Rust_(programming_language)".to_string());
        assert_eq!(res.url, expected_url);

        let cfg = Config {
            disable_json_ld: true,
            ..Default::default()
        };
        let mut readability = Readability::new(contents, None, Some(cfg)).unwrap();
        let res = readability.parse().unwrap();
        let expected_url = None;
        assert_eq!(res.url, expected_url);
    }

    #[test]
    fn test_max_elements() {
        let contents = include_str!("../test-pages/rustwiki_2024.html");
        // each element represent a test parameters, where 0 is max_elements_to_parse, 1 is want_err
        let tests = [(10, true), (0, false), (10000, false)];

        for (max_elements_to_parse, want_err) in tests {
            let cfg = Config {
                max_elements_to_parse,
                ..Default::default()
            };
            let mut readability = Readability::new(contents, None, Some(cfg)).unwrap();
            let res = readability.parse();
            if want_err {
                assert!(matches!(
                    res.err().unwrap(),
                    ReadabilityError::TooManyElements(_, _)
                ));
            } else {
                assert!(res.is_ok());
            }
        }
    }

    #[test]
    fn test_base_uri() {
        let contents = r#"<!DOCTYPE>
        <html>
            <head>
                <base href="https://example.com/">
                <title>Test</title>
            </head>
            <body>
            </body>
        </html>"#;
        let readability = Readability::from(contents);
        let base_url = readability.parse_base_url();
        assert_eq!(base_url.unwrap().as_str(), "https://example.com/");
    }
}
