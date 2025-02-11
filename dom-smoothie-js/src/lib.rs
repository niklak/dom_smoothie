mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature="mini-alloc")] {
        #[global_allocator]
        static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
    }
}

#[wasm_bindgen]
/// A struct that provides readability functionality
pub struct Readability(dom_smoothie::Readability);

#[wasm_bindgen]
impl Readability {
    #[wasm_bindgen(constructor)]
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
    /// - `cfg` -- an optional `JsValue` instance
    ///
    /// # Returns
    ///
    /// A new [`Readability`] instance
    ///
    /// # Errors
    ///
    /// Returns [`JsError`] if `document_url` is not a valid URL
    pub fn new(
        content: String,
        document_url: Option<String>,
        cfg: JsValue,
    ) -> Result<Readability, JsError> {
        let cfg = if cfg.is_null() {
            None
        } else {
            match serde_wasm_bindgen::from_value(cfg) {
                Ok(cfg) => Some(cfg),
                Err(e) => return Err(JsError::new(&e.to_string())),
            }
        };

        let doc_url = document_url.as_ref().map(|s| s.as_str());
        let ra = dom_smoothie::Readability::new(content, doc_url, cfg)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(Readability(ra))
    }

    /// Extracts the relevant content from the document and provides it as a JSON object.
    ///
    /// This is the primary method of the crate. It performs the following steps:
    ///
    /// - Verify the document
    /// - Extracts the metadata
    /// - Cleans the document
    /// - Extracts the main content of the document
    /// - Post-processes the content
    /// - Returns the content and the metadata as a JSON object
    ///
    /// # Returns
    ///
    /// A JSON object containing the content and the metadata.
    ///
    /// # Errors
    /// If `config.max_elements_to_parse` is > 0 and the document's number of element nodes exceeds this limit,
    /// a `JsError` error is returned.
    /// If the document fails to extract the content, a `JsError` error is returned.
    #[wasm_bindgen]
    pub fn parse(&mut self) -> Result<JsValue, JsError> {
        match self.0.parse() {
            Ok(article) => {
                serde_wasm_bindgen::to_value(&article).map_err(|e| JsError::new(&e.to_string()))
            }
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    /// Returns the title of the article as a string.
    #[wasm_bindgen]
    pub fn get_article_title(&mut self) -> String {
        self.0.get_article_title().to_string()
    }

    /// Searches for a JSON-LD block in the page and extracts the metadata from it.
    ///
    /// # Returns
    ///
    /// An object containing the metadata extracted from the JSON-LD block.
    /// If no valid JSON-LD block is found, this method returns `null`.
    #[wasm_bindgen]
    pub fn parse_json_ld(&mut self) -> JsValue {
        let json_ld = self.0.parse_json_ld();
        serde_wasm_bindgen::to_value(&json_ld)
            .ok()
            .unwrap_or(JsValue::null())
    }

    /// Extracts the metadata from the article.
    ///
    /// This method takes an optional `json_ld` object as input, which is used as a fallback
    /// if no metadata can be found on the page. If the input `json_ld` object contains any
    /// of the following fields, they will not be overwritten by this function:
    /// - `title`
    /// - `byline`
    /// - `excerpt`
    /// - `site_name`
    /// - `published_time`
    /// - `modified_time`
    /// - `lang`
    /// - `dir`
    /// - `image`
    /// - `url`
    ///
    /// # Returns
    ///
    /// An object containing the metadata extracted from the article.
    /// If no valid metadata can be found, this method returns `null`.
    #[wasm_bindgen]
    pub fn get_article_metadata(&mut self, json_ld: JsValue) -> JsValue {
        let json_ld: Option<dom_smoothie::Metadata> = serde_wasm_bindgen::from_value(json_ld).ok();
        let metadata = self.0.get_article_metadata(json_ld);
        serde_wasm_bindgen::to_value(&metadata)
            .ok()
            .unwrap_or(JsValue::null())
    }

    /// Returns true if the content is probably readable, false otherwise.
    ///
    /// This method is useful for quickly determining whether content is
    /// readable without having to parse the content.
    #[wasm_bindgen]
    pub fn is_probably_readable(&mut self) -> bool {
        self.0.is_probably_readable()
    }
}

/// Parse the content of a document.
///
/// This is a convenience method that is equivalent to creating a new
/// `Readability` instance and calling its `parse` method.
///
/// # Returns
///
/// An object containing the content and the metadata.
///
/// # Errors
///
/// Returns a `JsError` if the document fails to parse.
#[wasm_bindgen]
pub fn parse(content: &str) -> Result<JsValue, JsValue> {
    let mut ra = dom_smoothie::Readability::new(content, None, None)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let article = ra.parse().map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&article).map_err(|e| JsValue::from_str(&e.to_string()))
}
