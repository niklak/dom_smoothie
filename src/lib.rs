mod config;
mod glob;
mod grab;
mod grab_flags;
mod helpers;
mod prep_article;
mod readability;
mod readable;
mod score;

#[cfg(feature = "serde")]
mod serde_helpers;

pub use config::{CandidateSelectMode, Config, TextMode};
pub use readability::Article;
pub use readability::Metadata;
pub use readability::Readability;
pub use readable::is_probably_readable;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadabilityError {
    #[error(transparent)]
    BadDocumentURL(#[from] url::ParseError),
    #[error("failed to grab the article")]
    GrabFailed,
    #[error("too many elements in the document to parse (found {0}, maximum {1})")]
    TooManyElements(usize, usize),
}
