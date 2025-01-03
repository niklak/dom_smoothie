use std::error::Error;
use std::{fs, path::PathBuf};

use clap::Parser;
use dom_smoothie::{Readability, Article};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
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

    let mut ra = Readability::new(contents, document_url, None)?;
    let article = ra.parse()?;

    let Some(output_path) = cli.output else {
        unreachable!();
    };

    let result_html_path: PathBuf = output_path
        .clone()
        .join(format!("{}_result.html", input_fn.to_string_lossy()));

    fs::write(result_html_path, article.content.as_bytes())?;

    let result_text_path: PathBuf = output_path
        .clone()
        .join(format!("{}_result.txt", input_fn.to_string_lossy()));

    fs::write(result_text_path, article.text_content.as_bytes())?;

    let metadata = Metadata::from(&article);
    let metadata_content = serde_json::to_string_pretty(&metadata)?;

    let meta_path: PathBuf = output_path
        .clone()
        .join(format!("{}_metadata.json", input_fn.to_string_lossy()));

    fs::write(meta_path, metadata_content)?;

    Ok(())
}
