// lib.rs
use std::{
    fs::{self, File},
    io,
    path::Path,
};

use thiserror::Error;

pub mod formats;
pub use formats::Format;
pub mod universe;

#[derive(Error, Debug)]
pub enum ConwayError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Format detection error: {0}")]
    FormatDetection(String),

    #[error("Parsing error: {0}")]
    Parsing(String),

    #[error("Writing error: {0}")]
    Writing(String),
}

pub type Result<T> = std::result::Result<T, ConwayError>;

/// Detect the format of a Conway's Game of Life file
pub fn detect_format(content: &str) -> Result<Format> {
    // Check for RLE header signature
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        if formats::rle::is_valid_header(trimmed) {
            return Ok(Format::Rle);
        }

        return Ok(Format::Plaintext);
    }

    Err(ConwayError::FormatDetection(
        "Could not detect file format".into(),
    ))
}

/// Convert a file from one format to another
pub fn convert_file(input_path: &Path, output_path: &Path, force: bool) -> Result<()> {
    if !input_path.exists() {
        return Err(ConwayError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "Input file does not exist",
        )));
    }

    if output_path.exists() && !force {
        return Err(ConwayError::Io(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Output file already exists",
        )));
    }

    let content = fs::read_to_string(input_path)?;
    let input_format = detect_format(&content)?;

    let universe = formats::parse(&content, input_format)?;
    let target_format = match input_format {
        Format::Rle => Format::Plaintext,
        Format::Plaintext => Format::Rle,
    };

    let mut output_file = File::create(output_path)?;
    formats::write(&universe, &mut output_file, target_format)?;

    Ok(())
}

/// Removes comment lines from the pattern starting with the specified character
fn filter_comment_lines(content: &str, starting_char: char) -> Vec<&str> {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with(starting_char) && !line.is_empty())
        .collect()
}
