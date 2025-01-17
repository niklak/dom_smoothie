//! This is a reference implementation of a CLI tool for the `dom_smoothie` crate.
//!
//! The tool processes an HTML document using [`dom_smoothie::Readability`] to extract
//! relevant content and metadata. It accepts an input HTML file and outputs the
//! parsed article content as both HTML and plain text, along with metadata in JSON format.
//!
//! ## Usage
//! ```bash
//! dom_smoothie_cli --input path/to/input.html --output path/to/output/dir
//! ```
//!
//! If the `--output` argument is omitted, the results will be saved in the same directory
//! as the input file. An optional `--document-url` parameter can be provided to enhance
//! parsing accuracy by specifying the base document URL.

use std::error::Error;
use std::{fs, path::PathBuf};

use clap::Parser;
use dom_smoothie::{Article, Config, Readability};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(help_template = "{name} {version}\n\n{about}\n\n{usage}\n\n{all-args}")]
struct Cli {
    /// Sets an input path to the html document
    #[clap(short, long, value_parser)]
    input: PathBuf,
    /// Sets an output path. If omitted the parent dir of `<INPUT>` will be used.
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
    /// Sets an optional base document URL
    #[clap(short, long, value_parser, value_name = "URL")]
    document_url: Option<String>,
    /// Keeps elements' classes if set true
    #[clap(long, value_parser)]
    keep_classes: bool,
    /// Sets a list of classes that will be preserved and not removed during the post-process.
    /// Multiple classes should be separated by a comma (`,`)
    #[clap(long, value_parser, value_delimiter = ',')]
    preserved_classes: Vec<String>,
    /// Skips parsing metadata from ld+json script elements
    #[clap(long, value_parser)]
    disable_json_ld: bool,
    /// Sets a maximum number of elements to parse. If it equals 0, then there is no limit.
    #[clap(long, value_parser, default_value = "0")]
    max_elements: usize,
    /// Sets a character threshold for content extraction
    #[clap(long, value_parser, default_value = "500")]
    char_threshold: usize,
    /// Sets a number of top candidates for content extraction
    #[clap(long, value_parser, default_value = "5")]
    n_top_candidates: usize,
}

/// This struct represents the metadata from the [`dom_smoothie::Article`]
#[derive(Default, serde::Deserialize, serde::Serialize)]
struct Metadata {
    title: String,
    byline: Option<String>,
    excerpt: Option<String>,
    site_name: Option<String>,
    published_time: Option<String>,
    modified_time: Option<String>,
    lang: Option<String>,
    url: Option<String>,
    dir: Option<String>,
}

impl From<&Article> for Metadata {
    fn from(value: &Article) -> Self {
        Self {
            title: value.title.clone(),
            byline: value.byline.clone(),
            excerpt: value.excerpt.clone(),
            site_name: value.site_name.clone(),
            published_time: value.published_time.clone(),
            modified_time: value.modified_time.clone(),
            lang: value.lang.clone(),
            url: value.url.clone(),
            dir: value.dir.clone(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::parse();

    let input_path_buf = cli.input.with_extension("");
    let Some(input_fn) = input_path_buf.file_name() else {
        return Err("invalid input path".into());
    };

    if cli.output.is_none() {
        cli.output = input_path_buf.parent().map(|p| p.to_path_buf());
    }

    let contents = fs::read_to_string(cli.input)?;
    let document_url = cli.document_url.as_deref();

    let cfg = Config {
        keep_classes: cli.keep_classes,
        classes_to_preserve: cli.preserved_classes,
        max_elements_to_parse: cli.max_elements,
        disable_json_ld: cli.disable_json_ld,
        n_top_candidates: cli.n_top_candidates,
        char_threshold: cli.char_threshold,
        ..Default::default()
    };

    let mut ra = Readability::new(contents, document_url, Some(cfg))?;
    let article = ra.parse()?;

    let Some(output_path) = cli.output else {
        unreachable!();
    };

    let result_html_path: PathBuf =
        output_path.clone().join(format!("{}_result.html", input_fn.to_string_lossy()));

    fs::write(result_html_path, article.content.as_bytes())?;

    let result_text_path: PathBuf =
        output_path.clone().join(format!("{}_result.txt", input_fn.to_string_lossy()));

    fs::write(result_text_path, article.text_content.as_bytes())?;

    let metadata = Metadata::from(&article);
    let metadata_content = serde_json::to_string_pretty(&metadata)?;

    let meta_path: PathBuf =
        output_path.clone().join(format!("{}_metadata.json", input_fn.to_string_lossy()));

    fs::write(meta_path, metadata_content)?;

    Ok(())
}
