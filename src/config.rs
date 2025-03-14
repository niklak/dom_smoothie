use flagset::FlagSet;

use crate::{glob::{MIN_CONTENT_LENGTH, MIN_SCORE}, grab_flags::GrabFlags};

pub(crate) static DEFAULT_N_TOP_CANDIDATES: usize = 5;
pub(crate) static DEFAULT_CHAR_THRESHOLD: usize = 500;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy)]
pub enum CandidateSelectMode {
    #[default]
    Readability,
    DomSmoothie,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy)]
pub enum TextMode {
    #[default]
    Raw,
    Formatted,
    Markdown,
}

/// Configuration options for [`crate::Readability`]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
    /// Character threshold for content extraction
    pub char_threshold: usize,
    /// The minimum score required for the document to be considered readable. Defaults to 20.0.
    /// Used only for [`crate::Readability::is_probably_readable`].
    pub readable_min_score: f32,
    /// The minimum content length required for the document to be considered readable. Defaults to 140.
    /// Used only for [`crate::Readability::is_probably_readable`].
    pub readable_min_content_length: usize,
    /// Determines whether the top candidate is adjusted
    /// based on [Readability.js](https://github.com/mozilla/readability)
    /// or uses the crate's exclusive implementation.
    pub candidate_select_mode: CandidateSelectMode,
    /// Allows to set the text mode, whether it should be raw (as-is), formatted or markdown
    pub text_mode: TextMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keep_classes: false,
            classes_to_preserve: Vec::new(),
            max_elements_to_parse: 0,
            disable_json_ld: false,
            n_top_candidates: DEFAULT_N_TOP_CANDIDATES,
            char_threshold: DEFAULT_CHAR_THRESHOLD,
            readable_min_score: MIN_SCORE,
            readable_min_content_length: MIN_CONTENT_LENGTH,
            candidate_select_mode: CandidateSelectMode::Readability,
            text_mode: TextMode::Raw,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy)]
/// ParsePolicy is used to determine how much content cleaning should be applied to the extracted content.
pub enum  ParsePolicy {
    /// Strict policy -- removes unlikely elements before determining the elements score;
    /// uses id and class attributes of the element to determine it's score; 
    /// applies additional content cleaning after identifying the main content.
    #[default]
    Strict,
    /// Moderate policy, -- uses id and class attributes of the element to determine it's score; 
    /// applies additional content cleaning after identifying the main content.
    Moderate,
    /// Clean policy -- applies additional content cleaning after identifying the main content.
    Clean,
    /// Raw policy, applies no cleaning heuristics.
    Raw,
}

impl Into<FlagSet<GrabFlags>> for ParsePolicy {
    fn into(self) -> FlagSet<GrabFlags> {
        match self {
            ParsePolicy::Strict => FlagSet::full(),
            ParsePolicy::Moderate => GrabFlags::WeightClasses | GrabFlags::CleanConditionally,
            ParsePolicy::Clean => FlagSet::default() | GrabFlags::CleanConditionally,
            ParsePolicy::Raw => FlagSet::default(),
        }
    }
}