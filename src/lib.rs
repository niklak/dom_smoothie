mod glob;
mod grab;
mod grab_flags;
mod helpers;
mod prep_article;
mod readability;
mod score;

pub use readability::Config;
pub use readability::Metadata;
pub use readability::Readability;
pub use readability::Article;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadabilityError {
    #[error(transparent)]
    BadDocumentURL(#[from] url::ParseError),
    #[error("failed to grab the article")]
    GrabFailed,
}
