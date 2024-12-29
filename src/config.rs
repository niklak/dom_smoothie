pub(crate) static DEFAULT_N_TOP_CANDIDATES: usize = 5;

/// Configuration options for [`crate::Readability`]
pub struct Config {
    /// Set to `true` to keep all classes in the document
    pub keep_classes: bool,
    /// List of classes that will be preserved and not removed during the post-process.
    pub classes_to_preserve: Vec<String>,
    /// Maximum number of elements to parse
    pub max_elements_to_parse: usize,
    /// Disable JSON-LD extracting
    pub disable_json_ld: bool,
    /// Number of top candidates to handle
    pub n_top_candidates: usize,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            keep_classes: false,
            classes_to_preserve: Vec::new(),
            max_elements_to_parse: 0,
            disable_json_ld: false,
            n_top_candidates: DEFAULT_N_TOP_CANDIDATES,
        }
    }
}