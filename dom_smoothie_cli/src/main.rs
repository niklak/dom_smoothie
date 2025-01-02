use std::error::Error;
use std::{fs, path::PathBuf};

use clap::Parser;
use dom_smoothie::Readability;

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::parse();

    let input_path_buf = cli.input.clone();
    let Some(input_fn) = input_path_buf.file_name() else {
        return Err("invalid input path".into());
    };

    if cli.output.is_none() {
        cli.output = input_path_buf.parent().map(|p| p.to_path_buf());
    }

    let contents = fs::read_to_string(cli.input)?;
    let document_url = cli.document_url.as_deref();

    let mut ra = Readability::new(contents, document_url, None)?;
    let res = ra.parse()?;

    let Some(output_path) = cli.output else {
        unreachable!();
    };

    let result_html_path: PathBuf = output_path
        .clone()
        .join(format!("{}_result.html", input_fn.to_string_lossy()));

    fs::write(result_html_path, res.content.as_bytes())?;
    Ok(())
}
