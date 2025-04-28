use std::io::Write;

use crate::{universe::Universe, Result};

pub mod plaintext;
pub mod rle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Plaintext,
    Rle,
}

/// Parse a Game of Life file in the specified format
pub fn parse(content: &str, format: Format) -> Result<Universe> {
    match format {
        Format::Plaintext => plaintext::parse(content),
        Format::Rle => rle::parse(content),
    }
}

/// Write a Game of Life universe to a writer in the specified format
pub fn write(universe: &Universe, writer: &mut dyn Write, format: Format) -> Result<()> {
    match format {
        Format::Plaintext => plaintext::write(universe, writer),
        Format::Rle => rle::write(universe, writer),
    }
}
