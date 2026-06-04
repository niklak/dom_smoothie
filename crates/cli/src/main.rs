//! This is a reference implementation of a CLI tool for the `dom_smoothie` crate.
//!
//! The tool processes an HTML document using [`dom_smoothie::Readability`] to extract
//! relevant content and metadata. It accepts an input HTML file (or stdin) and outputs the
//! parsed article content as both HTML and plain text, along with metadata in JSON format.
//!
//! ## Usage
//! ```bash
//! # File input, file output (default)
//! dom_smoothie_cli --input path/to/input.html --output path/to/output/dir
//!
//! # Stdin to stdout
//! cat page.html | dom_smoothie_cli
//!
//! # Stdin, select text output
//! curl -s https://example.com | dom_smoothie_cli -f text
//!
//! # File input, stdout
//! dom_smoothie_cli --input page.html --stdout -f metadata
//! ```
//!
//! If the `--input` argument is omitted (or set to `-`), input is read from stdin.
//! When reading from stdin with no `--output` specified, results are printed to stdout.
//! If the `--output` argument is omitted with file input, the results will be saved in the
//! same directory as the input file. An optional `--document-url` parameter can be provided
//! to enhance parsing accuracy by specifying the base document URL.

use std::error::Error;
use std::ffi::OsString;
use std::io::{self, IsTerminal, Read, Write};
use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};
use dom_smoothie::{Article, CandidateSelectMode, Config, Readability, TextMode};

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    /// Extracted article HTML
    Html,
    /// Extracted plain text content
    Text,
    /// Article metadata as JSON
    Metadata,
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(help_template = "{name} {version}\n\n{about}\n\n{usage}\n\n{all-args}")]
struct Cli {
    /// Sets an input path to the html document. Omit or use `-` to read from stdin.
    #[clap(short, long, value_parser)]
    input: Option<PathBuf>,
    /// Sets an output path. If omitted the parent dir of `<INPUT>` will be used.
    /// When reading from stdin, omitting this enables stdout mode.
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
    /// Print output to stdout instead of writing files
    #[clap(long, value_parser)]
    stdout: bool,
    /// Output format when writing to stdout (html, text, or metadata)
    #[clap(short = 'f', long, value_enum, default_value = "html")]
    output_format: OutputFormat,
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
    // Produce formatted text output
    #[clap(long, value_parser, default_value = "false")]
    formatted_text: bool,
    // Use alternative (dom_smoothie) mode for finding common top candidate.
    #[clap(long, value_parser, default_value = "false")]
    alt_mode: bool,
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

/// Reads HTML content from either a file or stdin.
/// Returns the content string and an optional source name (file stem).
fn read_input(input: &Option<PathBuf>) -> Result<(String, Option<OsString>), Box<dyn Error>> {
    match input {
        Some(path) if path.as_os_str() != "-" => {
            let source_name = path.with_extension("").file_name().map(|n| n.to_owned());
            let contents = fs::read_to_string(path)?;
            Ok((contents, source_name))
        }
        _ => {
            let stdin = io::stdin();
            if stdin.is_terminal() {
                eprintln!(
                    "Warning: reading from terminal stdin. \
                     Did you mean to pipe input? Press Ctrl+D when done."
                );
            }
            let mut contents = String::new();
            stdin.lock().read_to_string(&mut contents)?;
            Ok((contents, None))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Read input from file or stdin
    let (contents, source_name) = read_input(&cli.input)?;
    let document_url = cli.document_url.as_deref();

    // Determine if we're in stdout mode:
    // explicitly via --stdout, or implicitly when input is stdin and no --output given
    let is_stdin = cli.input.is_none() || cli.input.as_deref() == Some(std::path::Path::new("-"));
    let use_stdout = cli.stdout || (is_stdin && cli.output.is_none());

    let text_mode = if cli.formatted_text {
        TextMode::Formatted
    } else {
        TextMode::Raw
    };

    let candidate_select_mode = if cli.alt_mode {
        CandidateSelectMode::DomSmoothie
    } else {
        CandidateSelectMode::Readability
    };

    let cfg = Config {
        keep_classes: cli.keep_classes,
        classes_to_preserve: cli.preserved_classes,
        max_elements_to_parse: cli.max_elements,
        disable_json_ld: cli.disable_json_ld,
        n_top_candidates: cli.n_top_candidates,
        char_threshold: cli.char_threshold,
        candidate_select_mode,
        text_mode,
        ..Default::default()
    };

    let mut ra = Readability::new(contents, document_url, Some(cfg))?;
    let article = ra.parse()?;

    if use_stdout {
        let mut stdout = io::stdout().lock();
        match cli.output_format {
            OutputFormat::Html => {
                write!(stdout, "{}", article.content)?;
            }
            OutputFormat::Text => {
                write!(stdout, "{}", article.text_content)?;
            }
            OutputFormat::Metadata => {
                let metadata = Metadata::from(&article);
                let metadata_content = serde_json::to_string_pretty(&metadata)?;
                write!(stdout, "{}", metadata_content)?;
            }
        }
    } else {
        // File output mode
        let base_name = source_name.unwrap_or_else(|| OsString::from("stdin"));
        let base_name_str = base_name.to_string_lossy();

        let output_path = cli.output.unwrap_or_else(|| {
            cli.input
                .as_ref()
                .and_then(|p| p.with_extension("").parent().map(|par| par.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."))
        });

        let result_html_path = output_path.join(format!("{base_name_str}_result.html"));
        fs::write(result_html_path, article.content.as_bytes())?;

        let result_text_path = output_path.join(format!("{base_name_str}_result.txt"));
        fs::write(result_text_path, article.text_content.as_bytes())?;

        let metadata = Metadata::from(&article);
        let metadata_content = serde_json::to_string_pretty(&metadata)?;
        let meta_path = output_path.join(format!("{base_name_str}_metadata.json"));
        fs::write(meta_path, metadata_content)?;
    }

    Ok(())
}
